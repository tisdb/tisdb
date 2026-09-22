// src/domain/node.rs
use crate::domain::id::NodeId;
use crate::domain::metadata::EntityHeader;
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq,
)]
#[rkyv(derive(Debug, PartialEq))]
pub struct Node {
    id: NodeId,
    pub header: EntityHeader,
}

impl Node {
    pub fn new(class_path: Vec<String>) -> Self {
        Self { 
            id: NodeId::new(),
            header: EntityHeader::new(class_path),
        }
    }

    #[inline]
    pub fn id(&self) -> NodeId {
        self.id
    }
}