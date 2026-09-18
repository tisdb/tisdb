// src/domain/hyper.rs
use crate::domain::id::{FlowId, HyperconnectorId, ZoneId};
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq, Eq,
)]
#[rkyv(derive(Debug, PartialEq, Eq))]
pub struct Hyperconnector {
    id: HyperconnectorId,
    zones: Vec<ZoneId>,
    flows: Vec<FlowId>,
}

impl Hyperconnector {
    pub fn new() -> Self {
        Self {
            id: HyperconnectorId::new(),
            zones: Vec::new(),
            flows: Vec::new(),
        }
    }

    #[inline]
    pub fn id(&self) -> HyperconnectorId {
        self.id
    }

    #[inline]
    pub fn zones(&self) -> &[ZoneId] {
        &self.zones
    }

    #[inline]
    pub fn flows(&self) -> &[FlowId] {
        &self.flows
    }

    pub fn add_zone(&mut self, zone_id: ZoneId) {
        if !self.zones.contains(&zone_id) {
            self.zones.push(zone_id);
        }
    }

    pub fn add_flow(&mut self, flow_id: FlowId) {
        if !self.flows.contains(&flow_id) {
            self.flows.push(flow_id);
        }
    }
}

impl Default for Hyperconnector {
    fn default() -> Self {
        Self::new()
    }
}