// src/engine.rs
use std::collections::{HashMap, HashSet};

use crate::domain::{
    EntityId, Flow, FlowId, FlowType, Hyperconnector, HyperconnectorId, Node, NodeId, Zone, ZoneId,
};
use crate::error::CoreError;
use crate::storage::StorageBackend;
use rkyv::util::AlignedVec;

pub struct CisowskiEngine<S: StorageBackend> {
    storage: S,
    nodes: HashMap<NodeId, Node>,
    hypers: HashMap<HyperconnectorId, Hyperconnector>,
    zones: HashMap<ZoneId, Zone>,
    flows: HashMap<FlowId, Flow>,
}

impl<S: StorageBackend> CisowskiEngine<S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            nodes: HashMap::new(),
            hypers: HashMap::new(),
            zones: HashMap::new(),
            flows: HashMap::new(),
        }
    }

    // --- AKSIOMAT 1: Tworzenie Atomowych Węzłów (N) ---
    pub fn create_node(&mut self, class_path: Vec<String>) -> NodeId {
        let node = Node::new(class_path);
        let id = node.id();
        self.nodes.insert(id, node);
        id
    }

    // --- AKSIOMAT 2 & 7: Tworzenie Hiperkonektora (H) ---
    pub fn create_hyperconnector(&mut self, class_path: Vec<String>) -> HyperconnectorId {
        let hyper = Hyperconnector::new(class_path);
        let id = hyper.id();
        self.hypers.insert(id, hyper);
        id
    }

    // --- AKSIOMAT 3 & 4: Tworzenie Strefy wewnątrz konkretnego Hiperkonektora ---
    pub fn create_zone(&mut self, parent_h: HyperconnectorId, class_path: Vec<String>) -> Result<ZoneId, CoreError> {
        let hyper = self
            .hypers
            .get_mut(&parent_h)
            .ok_or_else(|| CoreError::NotFound(parent_h.to_string()))?;

        let zone = Zone::new(parent_h, class_path);
        let zone_id = zone.id();

        self.zones.insert(zone_id, zone);
        hyper.add_zone(zone_id);

        Ok(zone_id)
    }

    // --- AKSIOMAT 8 & 9: Zawartość Strefy μZ oraz Weryfikacja Acykliczności Zagnieżdżenia ---
    pub fn add_entity_to_zone(
        &mut self,
        zone_id: ZoneId,
        entity: EntityId,
    ) -> Result<(), CoreError> {
        match entity {
            EntityId::Node(nid) => {
                if !self.nodes.contains_key(&nid) {
                    return Err(CoreError::NotFound(nid.to_string()));
                }
            }
            EntityId::Hyperconnector(hid) => {
                if !self.hypers.contains_key(&hid) {
                    return Err(CoreError::NotFound(hid.to_string()));
                }

                let target_zone = self
                    .zones
                    .get(&zone_id)
                    .ok_or_else(|| CoreError::NotFound(zone_id.to_string()))?;

                if self.is_transitively_contained(hid, target_zone.parent_hyper()) {
                    return Err(CoreError::Axiom9CycleDetected);
                }
            }
        }

        let zone = self
            .zones
            .get_mut(&zone_id)
            .ok_or_else(|| CoreError::NotFound(zone_id.to_string()))?;

        zone.add_entity(entity);
        Ok(())
    }

    pub fn remove_entity_from_zone(
        &mut self,
        zone_id: &ZoneId,
        entity: &EntityId,
    ) -> Result<(), CoreError> {
        let zone = self
            .zones
            .get_mut(zone_id)
            .ok_or_else(|| CoreError::NotFound(zone_id.to_string()))?;

        if !zone.remove_entity(entity) {
            return Err(CoreError::NotFound(format!(
                "Encja {:?} nie znajduje się w strefie {}",
                entity, zone_id
            )));
        }
        Ok(())
    }

    // --- AKSIOMAT 5, 6 & 10: Tworzenie Przepływu (F) ---
    pub fn create_flow(
        &mut self,
        parent_h: HyperconnectorId,
        source_z: ZoneId,
        target_z: ZoneId,
        flow_type: FlowType,
        class_path: Vec<String>,
    ) -> Result<FlowId, CoreError> {
        let src = self
            .zones
            .get(&source_z)
            .ok_or_else(|| CoreError::NotFound(source_z.to_string()))?;
        let tgt = self
            .zones
            .get(&target_z)
            .ok_or_else(|| CoreError::NotFound(target_z.to_string()))?;

        if src.parent_hyper() != parent_h || tgt.parent_hyper() != parent_h {
            return Err(CoreError::InvalidFlowBoundary);
        }

        let flow = Flow::new(parent_h, source_z, target_z, flow_type, class_path);
        let flow_id = flow.id();

        self.flows.insert(flow_id, flow);

        let hyper = self
            .hypers
            .get_mut(&parent_h)
            .ok_or_else(|| CoreError::NotFound(parent_h.to_string()))?;
        hyper.add_flow(flow_id);

        Ok(flow_id)
    }

    // ========================================================================
    // ŚCISŁE USUWANIE (RESTRICT / GUARDRAILS)
    // ========================================================================

    pub fn delete_node(&mut self, node_id: &NodeId) -> Result<(), CoreError> {
        let entity = EntityId::Node(*node_id);
        
        let usage_reason = self.zones.values().find_map(|zone| {
            if zone.contained_entities().contains(&entity) {
                Some(format!("Węzeł należy do Strefy {}", zone.id()))
            } else {
                None
            }
        });

        if let Some(reason) = usage_reason {
            return Err(CoreError::InUse(node_id.to_string(), reason));
        }

        self.nodes.remove(node_id).ok_or_else(|| CoreError::NotFound(node_id.to_string()))?;
        Ok(())
    }

    pub fn delete_zone(&mut self, zone_id: &ZoneId) -> Result<(), CoreError> {
        let zone = self.zones.get(zone_id).ok_or_else(|| CoreError::NotFound(zone_id.to_string()))?;

        if !zone.contained_entities().is_empty() {
            return Err(CoreError::InUse(
                zone_id.to_string(),
                "Strefa zawiera przypisane encje (najpierw odepnij zawartość)".into(),
            ));
        }

        let has_flows = self.flows.values().any(|f| f.source_zone() == *zone_id || f.target_zone() == *zone_id);
        if has_flows {
             return Err(CoreError::InUse(
                zone_id.to_string(),
                "Strefa posiada aktywne przepływy (najpierw usuń przepływy)".into(),
            ));
        }

        if let Some(hyper) = self.hypers.get_mut(&zone.parent_hyper()) {
            hyper.remove_zone(zone_id);
        }

        self.zones.remove(zone_id);
        Ok(())
    }

    pub fn delete_flow(&mut self, flow_id: &FlowId) -> Result<(), CoreError> {
        let flow = self
            .flows
            .remove(flow_id)
            .ok_or_else(|| CoreError::NotFound(flow_id.to_string()))?;

        if let Some(hyper) = self.hypers.get_mut(&flow.parent_hyper()) {
            hyper.remove_flow(flow_id);
        }

        Ok(())
    }

    pub fn delete_hyperconnector(&mut self, hyper_id: &HyperconnectorId) -> Result<(), CoreError> {
        let hyper = self.hypers.get(hyper_id).ok_or_else(|| CoreError::NotFound(hyper_id.to_string()))?;

        let entity = EntityId::Hyperconnector(*hyper_id);
        let nested_usage = self.zones.values().find_map(|z| {
             if z.contained_entities().contains(&entity) {
                 Some(format!("Hiperkonektor jest zagnieżdżony w Strefie {}", z.id()))
             } else {
                 None
             }
        });
        if let Some(reason) = nested_usage {
             return Err(CoreError::InUse(hyper_id.to_string(), reason));
        }

        for zone_id in hyper.zones() {
            if let Some(zone) = self.zones.get(zone_id) {
                if !zone.contained_entities().is_empty() {
                    return Err(CoreError::InUse(
                        hyper_id.to_string(),
                        format!("Hiperkonektor zawiera zajętą Strefę {}", zone_id),
                    ));
                }
            }
        }

        let zones_to_remove = hyper.zones().to_vec();
        for zone_id in zones_to_remove {
             self.zones.remove(&zone_id);
        }
        
        let flows_to_remove = hyper.flows().to_vec();
        for flow_id in flows_to_remove {
             self.flows.remove(&flow_id);
        }

        self.hypers.remove(hyper_id);
        Ok(())
    }

    fn is_transitively_contained(
        &self,
        ancestor: HyperconnectorId,
        target: HyperconnectorId,
    ) -> bool {
        if ancestor == target {
            return true;
        }

        let mut visited = HashSet::new();
        let mut stack = vec![ancestor];

        while let Some(current_h_id) = stack.pop() {
            if current_h_id == target {
                return true;
            }

            if !visited.insert(current_h_id) {
                continue;
            }

            if let Some(current_h) = self.hypers.get(&current_h_id) {
                for &z_id in current_h.zones() {
                    if let Some(zone) = self.zones.get(&z_id) {
                        for entity in zone.contained_entities() {
                            if let EntityId::Hyperconnector(child_h_id) = entity {
                                stack.push(*child_h_id);
                            }
                        }
                    }
                }
            }
        }

        false
    }

    pub fn get_node(&self, id: &NodeId) -> Option<&Node> { self.nodes.get(id) }
    pub fn get_node_mut(&mut self, id: &NodeId) -> Option<&mut Node> { self.nodes.get_mut(id) }

    pub fn get_hyperconnector(&self, id: &HyperconnectorId) -> Option<&Hyperconnector> { self.hypers.get(id) }
    pub fn get_hyperconnector_mut(&mut self, id: &HyperconnectorId) -> Option<&mut Hyperconnector> { self.hypers.get_mut(id) }

    pub fn get_zone(&self, id: &ZoneId) -> Option<&Zone> { self.zones.get(id) }
    pub fn get_zone_mut(&mut self, id: &ZoneId) -> Option<&mut Zone> { self.zones.get_mut(id) }

    pub fn get_flow(&self, id: &FlowId) -> Option<&Flow> { self.flows.get(id) }
    pub fn get_flow_mut(&mut self, id: &FlowId) -> Option<&mut Flow> { self.flows.get_mut(id) }
}

use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize, Debug)]
#[rkyv(derive(Debug))]
pub struct DatabaseSnapshot {
    pub nodes: Vec<Node>,
    pub hypers: Vec<Hyperconnector>,
    pub zones: Vec<Zone>,
    pub flows: Vec<Flow>,
}

impl<S: StorageBackend> CisowskiEngine<S> {
    pub fn save(&mut self) -> Result<(), CoreError> {
        let snapshot = DatabaseSnapshot {
            nodes: self.nodes.values().cloned().collect(),
            hypers: self.hypers.values().cloned().collect(),
            zones: self.zones.values().cloned().collect(),
            flows: self.flows.values().cloned().collect(),
        };

        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&snapshot)
            .map_err(|e| CoreError::SerializationError(format!("Write error: {}", e)))?;

        let mut payload = Vec::with_capacity(4 + bytes.len());
        payload.extend_from_slice(b"CDB1");
        payload.extend_from_slice(&bytes);
        
        // Zastąpiliśmy stare write_bytes i truncate jednym, bezpiecznym poleceniem
        self.storage.atomic_write_snapshot(&payload)?;
        
        Ok(())
    }

    pub fn load(&mut self) -> Result<(), CoreError> {
        let len = self.storage.len();
        if len == 0 {
            return Ok(());
        }
        
        if len < 4 {
             return Err(CoreError::StorageError(
                "Nieprawidłowy format pliku bazy (ucięty nagłówek)".into(),
            ));
        }

        let file_bytes = self.storage.read_bytes(0, len as usize)?;

        if &file_bytes[0..4] != b"CDB1" {
            return Err(CoreError::StorageError(
                "Nieprawidłowy format pliku bazy (brak magika CDB1)".into(),
            ));
        }

        let mut archive_bytes = AlignedVec::<16>::new();
        archive_bytes.extend_from_slice(&file_bytes[4..]);
        
        let archived = rkyv::access::<ArchivedDatabaseSnapshot, rkyv::rancor::Error>(archive_bytes.as_slice())
            .map_err(|e| CoreError::SerializationError(format!("Validation/Access error: {}", e)))?;

        let snapshot: DatabaseSnapshot = rkyv::deserialize::<
            DatabaseSnapshot,
            rkyv::rancor::Error,
        >(archived)
        .map_err(|e| CoreError::SerializationError(format!("Deserialize error: {}", e)))?;

        self.nodes = snapshot.nodes.into_iter().map(|n| (n.id(), n)).collect();
        self.hypers = snapshot.hypers.into_iter().map(|h| (h.id(), h)).collect();
        self.zones = snapshot.zones.into_iter().map(|z| (z.id(), z)).collect();
        self.flows = snapshot.flows.into_iter().map(|f| (f.id(), f)).collect();

        Ok(())
    }
}