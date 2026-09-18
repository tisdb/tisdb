// src/domain/node.rs
use crate::domain::id::NodeId;
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq, Eq,
)]
#[rkyv(derive(Debug, PartialEq, Eq))]
pub struct Node {
    id: NodeId,
}

impl Node {
    pub fn new() -> Self {
        Self { id: NodeId::new() }
    }

    pub fn with_id(id: NodeId) -> Self {
        Self { id }
    }

    #[inline]
    pub fn id(&self) -> NodeId {
        self.id
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}