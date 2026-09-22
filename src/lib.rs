pub mod error;

pub mod domain {
    pub mod flow;
    pub mod hyper;
    pub mod id;
    pub mod metadata;
    pub mod node;
    pub mod zone;

    pub use flow::{Flow, FlowType};
    pub use hyper::Hyperconnector;
    pub use id::{EntityId, FlowId, HyperconnectorId, NodeId, ZoneId};
    pub use metadata::{AttributeValue, EntityHeader, ScalarValue, TextFormat};
    pub use node::Node;
    pub use zone::Zone;
}

pub mod storage {
    pub mod backend;
    pub mod file;
    pub mod memory;

    pub use backend::StorageBackend;
    pub use file::FileStorage;
    pub use memory::MemoryStorage;
}

pub mod engine;

// Re-eksporty dla wygody używania crate'a na zewnątrz
pub use domain::*;
pub use engine::CisowskiEngine;
pub use error::CoreError;
pub use storage::*;