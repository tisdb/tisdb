use std::fs::{self, OpenOptions};
use std::io::Write;

use tisdb::{CisowskiEngine, CoreError, FileStorage};

#[test]
fn test_file_storage_lifecycle() {
    let db_path = "test_graph.cdb";
    let lock_path = "test_graph.cdb.lock";

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    {
        let storage = FileStorage::open(db_path).expect("Nie udało się utworzyć magazynu plików");
        let mut engine = CisowskiEngine::new(storage);

        let _node_id = engine.create_node(vec![]);
        let _hyper_id = engine.create_hyperconnector(vec![]);
    }

    assert!(fs::metadata(db_path).is_ok());

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}

#[test]
fn test_broken_header() {
    let db_path = "test_broken.cdb";
    let lock_path = "test_broken.cdb.lock";
    
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    // Tworzymy "uszkodzony" plik
    {
        let mut file = OpenOptions::new().write(true).create(true).open(db_path).unwrap();
        file.write_all(b"CDB").unwrap(); // Tylko 3 bajty, za mało na nagłówek
    }

    {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);
        let result = engine.load();
        
        match result {
             Err(CoreError::StorageError(msg)) => assert!(msg.contains("ucięty nagłówek")),
             _ => panic!("System nie wykrył uszkodzonego nagłówka"),
        }
    }

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}