use std::fs;
use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use rkyv::{Archive, Deserialize, Serialize};
use rkyv::rancor::Error as RancorError;

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
#[rkyv(derive(Debug, PartialEq))]
pub struct DummyNode {
    pub id: u64,
    pub class: String,
}

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
#[rkyv(derive(Debug, PartialEq))]
pub struct DummyZone {
    pub id: u64,
    pub name: String,
}

const NODES_TABLE: TableDefinition<u64, &[u8]> = TableDefinition::new("nodes");
const ZONES_TABLE: TableDefinition<u64, &[u8]> = TableDefinition::new("zones");
const METADATA_TABLE: TableDefinition<&str, u32> = TableDefinition::new("metadata");

#[test]
fn test_redb_prototype_atomicity_and_unaligned_rkyv_integration() {
    let db_path = "test_redb_unaligned.db";
    let _ = fs::remove_file(db_path); 

    // KROK 1: Inicjalizacja i atomowa transakcja zapisu
    {
        let db = Database::create(db_path).unwrap();
        let write_txn = db.begin_write().unwrap();
        
        {
            let mut node_table = write_txn.open_table(NODES_TABLE).unwrap();
            let mut meta_table = write_txn.open_table(METADATA_TABLE).unwrap();

            meta_table.insert("format_version", &2).unwrap();

            let node1 = DummyNode { id: 101, class: "Proces".into() };
            let bytes1 = rkyv::to_bytes::<RancorError>(&node1).unwrap();
            node_table.insert(101, bytes1.as_slice()).unwrap();
        } 
        
        write_txn.commit().unwrap();
    }

    // KROK 2: Odczyt transakcyjny
    {
        let db = Database::create(db_path).unwrap(); 
        let read_txn = db.begin_read().unwrap();
        
        let node_table = read_txn.open_table(NODES_TABLE).unwrap();
        
        let guard = node_table.get(101).unwrap().unwrap();
        let node1_bytes = guard.value();

        let archived_node = rkyv::access::<ArchivedDummyNode, RancorError>(node1_bytes)
            .expect("Rkyv odrzucił bajty z bazy redb - sprawdz wyrównanie (unaligned)!");
            
        assert_eq!(archived_node.id, 101);
        
        let loaded_node: DummyNode = rkyv::deserialize::<DummyNode, RancorError>(archived_node).unwrap();
        assert_eq!(loaded_node.id, 101);
    }

    let _ = fs::remove_file(db_path);
}

#[test]
fn test_redb_rollback_atomicity() {
    let db_path = "test_redb_rollback.db";
    let _ = fs::remove_file(db_path); 

    // 1. Zapis stabilny (Stan początkowy)
    {
        let db = Database::create(db_path).unwrap();
        let write_txn = db.begin_write().unwrap();
        {
            let mut meta_table = write_txn.open_table(METADATA_TABLE).unwrap();
            meta_table.insert("format_version", &2).unwrap();
            
            // NOWE: Inicjalizujemy puste tabele, aby istniały dla transakcji odczytu
            let _ = write_txn.open_table(NODES_TABLE).unwrap();
            let _ = write_txn.open_table(ZONES_TABLE).unwrap();
        }
        write_txn.commit().unwrap();
    }

    // 2. Próba wielotabelowego zapisu (Symulacja przerwania / Abort)
    {
        let db = Database::create(db_path).unwrap();
        let write_txn = db.begin_write().unwrap();
        {
            let mut node_table = write_txn.open_table(NODES_TABLE).unwrap();
            let mut zone_table = write_txn.open_table(ZONES_TABLE).unwrap();
            
            // Częściowe dodanie danych
            let n = DummyNode { id: 777, class: "A".into() };
            let z = DummyZone { id: 888, name: "B".into() };
            
            node_table.insert(777, rkyv::to_bytes::<RancorError>(&n).unwrap().as_slice()).unwrap();
            zone_table.insert(888, rkyv::to_bytes::<RancorError>(&z).unwrap().as_slice()).unwrap();
            
            // Brak commit(). drop(write_txn) robi automatyczny rollback.
        }
    }

    // 3. Weryfikacja: Częściowe dane NIE MOGĄ znajdować się na dysku
    {
        let db = Database::create(db_path).unwrap();
        let read_txn = db.begin_read().unwrap();
        
        // Teraz open_table nie wyrzuci błędu, bo tabele zostały utworzone w kroku 1.
        let node_table = read_txn.open_table(NODES_TABLE).unwrap();
        let zone_table = read_txn.open_table(ZONES_TABLE).unwrap();
        
        assert!(node_table.get(777).unwrap().is_none(), "Baza naruszyła izolację - Node istnieje!");
        assert!(zone_table.get(888).unwrap().is_none(), "Baza naruszyła izolację - Zone istnieje!");
        
        let meta_table = read_txn.open_table(METADATA_TABLE).unwrap();
        assert_eq!(meta_table.get("format_version").unwrap().unwrap().value(), 2);
    }
    
    let _ = fs::remove_file(db_path); 
}