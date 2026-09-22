// src/domain/hyper.rs
use crate::domain::id::{FlowId, HyperconnectorId, ZoneId};
use crate::domain::metadata::EntityHeader;
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq,
)]
#[rkyv(derive(Debug, PartialEq))]
pub struct Hyperconnector {
    id: HyperconnectorId,
    zones: Vec<ZoneId>,
    flows: Vec<FlowId>,
    pub header: EntityHeader,
}

impl Hyperconnector {
    pub fn new(class_path: Vec<String>) -> Self {
        Self {
            id: HyperconnectorId::new(),
            zones: Vec::new(),
            flows: Vec::new(),
            header: EntityHeader::new(class_path),
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
            // Uwaga: Zmiana topologii niekoniecznie oznacza zmianę semantyki (atrybutów) 
            // samego H, ale dla OCC można by tutaj wywoływać bump_revision. Na razie pomijamy.
        }
    }

    pub fn add_flow(&mut self, flow_id: FlowId) {
        if !self.flows.contains(&flow_id) {
            self.flows.push(flow_id);
        }
    }

    pub fn remove_zone(&mut self, zone_id: &ZoneId) -> bool {
        if let Some(pos) = self.zones.iter().position(|z| z == zone_id) {
            self.zones.swap_remove(pos);
            true
        } else {
            false
        }
    }

    pub fn remove_flow(&mut self, flow_id: &FlowId) -> bool {
        if let Some(pos) = self.flows.iter().position(|f| f == flow_id) {
            self.flows.swap_remove(pos);
            true
        } else {
            false
        }
    }
}