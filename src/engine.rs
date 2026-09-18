use std::collections::{HashMap, HashSet};

use crate::domain::{
    EntityId, Flow, FlowId, FlowType, Hyperconnector, HyperconnectorId, Node, NodeId, Zone, ZoneId,
};
use crate::error::CoreError;
use crate::storage::StorageBackend;

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
    pub fn create_node(&mut self) -> NodeId {
        let node = Node::new();
        let id = node.id();
        self.nodes.insert(id, node);
        id
    }

    // --- AKSIOMAT 2 & 7: Tworzenie Hiperkonektora (H) ---
    pub fn create_hyperconnector(&mut self) -> HyperconnectorId {
        let hyper = Hyperconnector::new();
        let id = hyper.id();
        self.hypers.insert(id, hyper);
        id
    }

    // --- AKSIOMAT 3 & 4: Tworzenie Strefy wewnątrz konkretnego Hiperkonektora ---
    pub fn create_zone(&mut self, parent_h: HyperconnectorId) -> Result<ZoneId, CoreError> {
        let hyper = self
            .hypers
            .get_mut(&parent_h)
            .ok_or_else(|| CoreError::NotFound(parent_h.to_string()))?;

        let zone = Zone::new(parent_h);
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
        // 1. Sprawdzenie istnienia encji w uniwersum U = N ⊔ H
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

                // 2. Walidacja Aksjomatu 9: zapobieganie cyklom zagnieżdżania h ∈ μZ(z_h)
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

    // --- AKSIOMAT 5, 6 & 10: Tworzenie Przepływu (F) ---
    pub fn create_flow(
        &mut self,
        parent_h: HyperconnectorId,
        source_z: ZoneId,
        target_z: ZoneId,
        flow_type: FlowType,
    ) -> Result<FlowId, CoreError> {
        // Walidacja: Czy obie strefy istnieją i należą do tego samego hiperkonektora macierzystego
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

        let flow = Flow::new(parent_h, source_z, target_z, flow_type);
        let flow_id = flow.id();

        self.flows.insert(flow_id, flow);

        let hyper = self
            .hypers
            .get_mut(&parent_h)
            .ok_or_else(|| CoreError::NotFound(parent_h.to_string()))?;
        hyper.add_flow(flow_id);

        Ok(flow_id)
    }

    // Pomocniczy algorytm detekcji cyklu dla Aksjomatu 9 (DFS)
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

    // Gettery odczytu
    pub fn get_node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn get_hyperconnector(&self, id: &HyperconnectorId) -> Option<&Hyperconnector> {
        self.hypers.get(id)
    }

    pub fn get_zone(&self, id: &ZoneId) -> Option<&Zone> {
        self.zones.get(id)
    }

    pub fn get_flow(&self, id: &FlowId) -> Option<&Flow> {
        self.flows.get(id)
    }
}

use rkyv::{Archive, Deserialize, Serialize};

/// Reprezentacja binarnej migawki stanu całej bazy (.cdb)
#[derive(Archive, Serialize, Deserialize, Debug)]
#[rkyv(derive(Debug))]
pub struct DatabaseSnapshot {
    pub nodes: Vec<Node>,
    pub hypers: Vec<Hyperconnector>,
    pub zones: Vec<Zone>,
    pub flows: Vec<Flow>,
}

impl<S: StorageBackend> CisowskiEngine<S> {
    /// Zapisuje pełny stan silnika do połączonego magazynu I/O
    pub fn save(&mut self) -> Result<(), CoreError> {
        let snapshot = DatabaseSnapshot {
            nodes: self.nodes.values().cloned().collect(),
            hypers: self.hypers.values().cloned().collect(),
            zones: self.zones.values().cloned().collect(),
            flows: self.flows.values().cloned().collect(),
        };

        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&snapshot)
            .map_err(|_| CoreError::SerializationError)?;

        // Nagłówek binarny CDB: Magic Bytes [C, D, B, 1] + Długość
        let mut payload = Vec::with_capacity(4 + bytes.len());
        payload.extend_from_slice(b"CDB1");
        payload.extend_from_slice(&bytes);

        self.storage.write_bytes(0, &payload)?;
        self.storage.flush()?;
        Ok(())
    }

    /// Wczytuje i odtwarza stan silnika z magazynu I/O
    pub fn load(&mut self) -> Result<(), CoreError> {
        let len = self.storage.len();
        if len < 4 {
            return Ok(()); // Pusta baza
        }

        let bytes = self.storage.read_bytes(0, len as usize)?;

        if &bytes[0..4] != b"CDB1" {
            return Err(CoreError::StorageError(
                "Nieprawidłowy format pliku bazy (brak magika CDB1)".into(),
            ));
        }

        let archived = rkyv::access::<ArchivedDatabaseSnapshot, rkyv::rancor::Error>(&bytes[4..])
            .map_err(|_| CoreError::SerializationError)?;

        let snapshot: DatabaseSnapshot = rkyv::deserialize::<
            DatabaseSnapshot,
            rkyv::rancor::Error,
        >(archived)
        .map_err(|_| CoreError::SerializationError)?;

        self.nodes = snapshot.nodes.into_iter().map(|n| (n.id(), n)).collect();
        self.hypers = snapshot
            .hypers
            .into_iter()
            .map(|h| (h.id(), h))
            .collect();
        self.zones = snapshot.zones.into_iter().map(|z| (z.id(), z)).collect();
        self.flows = snapshot.flows.into_iter().map(|f| (f.id(), f)).collect();

        Ok(())
    }
}

