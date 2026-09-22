use std::fs;
use tisdb::{AttributeValue, CisowskiEngine, EntityId, FileStorage, ScalarValue, TextFormat};

#[test]
fn test_binary_save_and_load_cycle() {
    let db_path = "test_persistence.cdb";
    let lock_path = "test_persistence.cdb.lock";

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    let (node_id, hyper_id, zone_id) = {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);

        let nid = engine.create_node(vec!["PersistNode".into()]);
        let hid = engine.create_hyperconnector(vec!["PersistHyper".into()]);
        let zid = engine.create_zone(hid, vec![]).unwrap();

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
        
        let node = engine.get_node(&node_id).unwrap();
        assert_eq!(node.header.class_path()[0], "PersistNode");
    }

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}

#[test]
fn test_truncate_on_shrink() {
    let db_path = "test_truncate.cdb";
    let lock_path = "test_truncate.cdb.lock";
    
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    // KROK 1: Zapis dużej bazy
    let size_large = {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);
        for _ in 0..100 {
             engine.create_node(vec![]);
        }
        engine.save().unwrap();
        fs::metadata(db_path).unwrap().len()
    };
    
    // KROK 2: Nadpisanie małą bazą
    let size_small = {
        let storage = FileStorage::open(db_path).unwrap(); // Otwiera istniejacy plik
        let mut engine = CisowskiEngine::new(storage);
        // Nadpisuje 100 węzłów z pamięci pustym grafem
        engine.save().unwrap();
        fs::metadata(db_path).unwrap().len()
    };
    
    assert!(size_small < size_large, "Plik powinien zostać skrócony (truncated)");
    
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}

#[test]
fn test_attributes_persistence_hashmap() {
    let db_path = "test_attrs.cdb";
    let lock_path = "test_attrs.cdb.lock";
    
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    let node_id = {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);
        let nid = engine.create_node(vec![]);
        
        let node = engine.get_node_mut(&nid).unwrap();
        node.header.set_attribute("wiek", AttributeValue::Integer(99));
        node.header.set_attribute("opis", AttributeValue::RichText { 
            format: TextFormat::Markdown, 
            content: "**Test**".to_string() 
        });
        node.header.set_attribute("tagi", AttributeValue::Set(vec![
             ScalarValue::String("tag1".into()),
             ScalarValue::String("tag2".into()),
        ]));
        
        engine.save().unwrap();
        nid
    };

    {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);
        engine.load().unwrap();
        
        let node = engine.get_node(&node_id).unwrap();
        assert_eq!(node.header.get_attribute("wiek"), Some(&AttributeValue::Integer(99)));
        
        match node.header.get_attribute("opis") {
            Some(AttributeValue::RichText { format, content }) => {
                assert_eq!(format, &TextFormat::Markdown);
                assert_eq!(content, "**Test**");
            },
            _ => panic!("Błąd persystencji RichText"),
        }
        
        match node.header.get_attribute("tagi") {
            Some(AttributeValue::Set(vec)) => {
                assert_eq!(vec.len(), 2);
            },
            _ => panic!("Błąd persystencji Set"),
        }
    }
    
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}