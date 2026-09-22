use std::path::Path;

use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition, ReadTransaction, WriteTransaction};
use rkyv::{rancor::Error as RancorError, Archive};

use crate::domain::{
    Flow, FlowId, Hyperconnector, HyperconnectorId, Node, NodeId, Zone, ZoneId
};
use crate::error::CoreError;

// 1. Definicje fizycznych tabel wewnątrz pliku B-Tree
// Zastosowanie tablic [u8; 16] dla kluczy gwarantuje doskonałą wydajność indeksów redb (stała długość)
const NODES_TABLE: TableDefinition<[u8; 16], &[u8]> = TableDefinition::new("nodes");
const HYPERS_TABLE: TableDefinition<[u8; 16], &[u8]> = TableDefinition::new("hypers");
const ZONES_TABLE: TableDefinition<[u8; 16], &[u8]> = TableDefinition::new("zones");
const FLOWS_TABLE: TableDefinition<[u8; 16], &[u8]> = TableDefinition::new("flows");
const META_TABLE: TableDefinition<&str, u32> = TableDefinition::new("metadata");

pub struct RedbStorage {
    db: Database,
}

impl RedbStorage {
    /// Otwiera bazę i natychmiast inicjalizuje schemat, co chroni przed błędem "TableDoesNotExist"
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, CoreError> {
        let db = Database::create(path)
            .map_err(|e| CoreError::StorageError(format!("Błąd otwarcia bazy redb: {}", e)))?;
        
        let write_txn = db.begin_write().map_err(|e| CoreError::StorageError(e.to_string()))?;
        {
            let _ = write_txn.open_table(NODES_TABLE).unwrap();
            let _ = write_txn.open_table(HYPERS_TABLE).unwrap();
            let _ = write_txn.open_table(ZONES_TABLE).unwrap();
            let _ = write_txn.open_table(FLOWS_TABLE).unwrap();
            
            let mut meta = write_txn.open_table(META_TABLE).unwrap();
            if meta.get("format_version").unwrap().is_none() {
                meta.insert("format_version", &2).unwrap(); // Zgodnie z formatem v2 (unaligned)
            }
        }
        write_txn.commit().map_err(|e| CoreError::StorageError(e.to_string()))?;

        Ok(Self { db })
    }

    /// Otwiera izolowaną transakcję zapisu (blokuje innych pisarzy)
    pub fn begin_write(&self) -> Result<RedbWriteTxn, CoreError> {
        let txn = self.db.begin_write().map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(RedbWriteTxn { txn })
    }

    /// Otwiera bez-blokadową transakcję odczytu ze zrzutu MVCC bazy (Snapshot Isolation)
    pub fn begin_read(&self) -> Result<RedbReadTxn, CoreError> {
        let txn = self.db.begin_read().map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(RedbReadTxn { txn })
    }
}

// =====================================================================
// TRANSAKCJA ODCZYTU (READ-ONLY)
// =====================================================================
pub struct RedbReadTxn {
    txn: ReadTransaction,
}

impl RedbReadTxn {
    pub fn get_node(&self, id: &NodeId) -> Result<Option<Node>, CoreError> {
        let table = self.txn.open_table(NODES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Node as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Node, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    pub fn get_hyperconnector(&self, id: &HyperconnectorId) -> Result<Option<Hyperconnector>, CoreError> {
        let table = self.txn.open_table(HYPERS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Hyperconnector as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Hyperconnector, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    pub fn get_zone(&self, id: &ZoneId) -> Result<Option<Zone>, CoreError> {
        let table = self.txn.open_table(ZONES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Zone as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Zone, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    pub fn get_flow(&self, id: &FlowId) -> Result<Option<Flow>, CoreError> {
        let table = self.txn.open_table(FLOWS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Flow as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Flow, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }
}

// =====================================================================
// TRANSAKCJA ZAPISU (READ-WRITE)
// =====================================================================
pub struct RedbWriteTxn {
    txn: WriteTransaction,
}

impl RedbWriteTxn {
    // --- NODE ---
    pub fn get_node(&self, id: &NodeId) -> Result<Option<Node>, CoreError> {
        let table = self.txn.open_table(NODES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Node as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Node, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    pub fn put_node(&self, node: &Node) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(NODES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        let bytes = rkyv::to_bytes::<RancorError>(node).map_err(|e| CoreError::SerializationError(e.to_string()))?;
        table.insert(*node.id().as_bytes(), bytes.as_slice()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }
    
    pub fn delete_node(&self, id: &NodeId) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(NODES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        table.remove(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }

    // --- HYPERCONNECTOR ---
    pub fn get_hyperconnector(&self, id: &HyperconnectorId) -> Result<Option<Hyperconnector>, CoreError> {
        let table = self.txn.open_table(HYPERS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Hyperconnector as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Hyperconnector, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    pub fn put_hyperconnector(&self, hyper: &Hyperconnector) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(HYPERS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        let bytes = rkyv::to_bytes::<RancorError>(hyper).map_err(|e| CoreError::SerializationError(e.to_string()))?;
        table.insert(*hyper.id().as_bytes(), bytes.as_slice()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }
    
    pub fn delete_hyperconnector(&self, id: &HyperconnectorId) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(HYPERS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        table.remove(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }

    // --- ZONE ---
    pub fn get_zone(&self, id: &ZoneId) -> Result<Option<Zone>, CoreError> {
        let table = self.txn.open_table(ZONES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Zone as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Zone, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    pub fn put_zone(&self, zone: &Zone) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(ZONES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        let bytes = rkyv::to_bytes::<RancorError>(zone).map_err(|e| CoreError::SerializationError(e.to_string()))?;
        table.insert(*zone.id().as_bytes(), bytes.as_slice()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }

    pub fn delete_zone(&self, id: &ZoneId) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(ZONES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        table.remove(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }

    // --- FLOW ---
    pub fn get_flow(&self, id: &FlowId) -> Result<Option<Flow>, CoreError> {
        let table = self.txn.open_table(FLOWS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Flow as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Flow, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    pub fn put_flow(&self, flow: &Flow) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(FLOWS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        let bytes = rkyv::to_bytes::<RancorError>(flow).map_err(|e| CoreError::SerializationError(e.to_string()))?;
        table.insert(*flow.id().as_bytes(), bytes.as_slice()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }

    pub fn delete_flow(&self, id: &FlowId) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(FLOWS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        table.remove(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }

    // --- Zatwierdzenie (ACID Commit) ---
    pub fn commit(self) -> Result<(), CoreError> {
        self.txn.commit().map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }
}