use tisdb::{CisowskiEngine, CoreError, EntityId, FlowType, MemoryStorage};

#[test]
fn test_node_and_hyperconnector_creation() {
    let storage = MemoryStorage::new();
    let mut engine = CisowskiEngine::new(storage);

    let node_id = engine.create_node(vec!["TestNode".into()]);
    let hyper_id = engine.create_hyperconnector(vec!["TestHyper".into()]);

    assert!(engine.get_node(&node_id).is_some());
    assert!(engine.get_hyperconnector(&hyper_id).is_some());
}

#[test]
fn test_axiom_3_and_4_zone_creation() {
    let storage = MemoryStorage::new();
    let mut engine = CisowskiEngine::new(storage);

    let hyper_id = engine.create_hyperconnector(vec![]);
    let zone_id = engine.create_zone(hyper_id, vec![]).unwrap();

    let zone = engine.get_zone(&zone_id).unwrap();
    assert_eq!(zone.parent_hyper(), hyper_id);
}

#[test]
fn test_axiom_9_cycle_prevention() {
    let storage = MemoryStorage::new();
    let mut engine = CisowskiEngine::new(storage);

    let h1 = engine.create_hyperconnector(vec![]);
    let z1 = engine.create_zone(h1, vec![]).unwrap();

    let h2 = engine.create_hyperconnector(vec![]);
    let z2 = engine.create_zone(h2, vec![]).unwrap();

    engine
        .add_entity_to_zone(z1, EntityId::Hyperconnector(h2))
        .unwrap();

    let result = engine.add_entity_to_zone(z2, EntityId::Hyperconnector(h1));
    assert_eq!(result, Err(CoreError::Axiom9CycleDetected));
}

#[test]
fn test_axiom_10_flow_boundaries() {
    let storage = MemoryStorage::new();
    let mut engine = CisowskiEngine::new(storage);

    let h1 = engine.create_hyperconnector(vec![]);
    let z1 = engine.create_zone(h1, vec![]).unwrap();

    let h2 = engine.create_hyperconnector(vec![]);
    let z2 = engine.create_zone(h2, vec![]).unwrap();

    let result = engine.create_flow(h1, z1, z2, FlowType::Dir, vec![]);
    assert_eq!(result, Err(CoreError::InvalidFlowBoundary));
}