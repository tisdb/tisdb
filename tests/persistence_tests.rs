use std::fs;

use cisowski_core_db::{CisowskiEngine, EntityId, FileStorage};

#[test]
fn test_binary_save_and_load_cycle() {
    let db_path = "test_persistence.cdb";
    let lock_path = "test_persistence.cdb.lock";

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    let (node_id, hyper_id, zone_id) = {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);

        let nid = engine.create_node();
        let hid = engine.create_hyperconnector();
        let zid = engine.create_zone(hid).unwrap();

        engine
            .add_entity_to_zone(zid, EntityId::Node(nid))
            .unwrap();

        engine.save().expect("Błąd podczas zapisu migawki");
        (nid, hid, zid)
    };

    // Ponowne otwarcie tej samej bazy z dysku
    {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);
        engine.load().expect("Błąd podczas odczytu migawki");

        assert!(engine.get_node(&node_id).is_some());
        assert!(engine.get_hyperconnector(&hyper_id).is_some());

        let zone = engine.get_zone(&zone_id).unwrap();
        assert_eq!(zone.contained_entities().len(), 1);
        assert_eq!(zone.contained_entities()[0], EntityId::Node(node_id));
    }

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}