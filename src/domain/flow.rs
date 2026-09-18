use crate::domain::id::{FlowId, HyperconnectorId, ZoneId};
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

#[derive(
    Archive,
    Serialize,
    Deserialize,
    SerdeSerialize,
    SerdeDeserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
#[repr(u8)]
pub enum FlowType {
    Sym = 0,   // Nieskierowane (statyczne)
    Dir = 1,   // Skierowane
    Bidir = 2, // Dwukierunkowe (aktywne)
}

#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq, Eq,
)]
#[rkyv(derive(Debug, PartialEq, Eq))]
pub struct Flow {
    id: FlowId,
    parent_hyper: HyperconnectorId,
    source_zone: ZoneId,
    target_zone: ZoneId,
    flow_type: FlowType,
}

impl Flow {
    pub fn new(
        parent_hyper: HyperconnectorId,
        source_zone: ZoneId,
        target_zone: ZoneId,
        flow_type: FlowType,
    ) -> Self {
        Self {
            id: FlowId::new(),
            parent_hyper,
            source_zone,
            target_zone,
            flow_type,
        }
    }

    #[inline]
    pub fn id(&self) -> FlowId {
        self.id
    }
    #[inline]
    pub fn parent_hyper(&self) -> HyperconnectorId {
        self.parent_hyper
    }
    #[inline]
    pub fn source_zone(&self) -> ZoneId {
        self.source_zone
    }
    #[inline]
    pub fn target_zone(&self) -> ZoneId {
        self.target_zone
    }
    #[inline]
    pub fn flow_type(&self) -> FlowType {
        self.flow_type
    }
}