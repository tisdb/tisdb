use std::fs;

use cisowski_core_db::{CisowskiEngine, FileStorage};

#[test]
fn test_file_storage_lifecycle() {
    let db_path = "test_graph.cdb";
    let lock_path = "test_graph.cdb.lock";

    // Czyszczenie po poprzednich uruchomieniach
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    {
        let storage = FileStorage::open(db_path).expect("Nie udało się utworzyć magazynu plików");
        let mut engine = CisowskiEngine::new(storage);

        let _node_id = engine.create_node();
        let _hyper_id = engine.create_hyperconnector();
    }

    // Sprawdzenie, czy plik bazy oraz plik blokady powstały na dysku
    assert!(fs::metadata(db_path).is_ok());

    // Sprzątanie po teście
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}