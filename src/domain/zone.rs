use crate::domain::id::{EntityId, HyperconnectorId, ZoneId};
use crate::domain::metadata::EntityHeader;
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq,
)]
#[rkyv(derive(Debug, PartialEq))]
pub struct Zone {
    id: ZoneId,
    parent_hyper: HyperconnectorId,
    contained_entities: Vec<EntityId>,
    pub header: EntityHeader,
}

impl Zone {
    pub fn new(parent_hyper: HyperconnectorId, class_path: Vec<String>) -> Self {
        Self {
            id: ZoneId::new(),
            parent_hyper,
            contained_entities: Vec::new(),
            header: EntityHeader::new(class_path),
        }
    }

    #[inline]
    pub fn id(&self) -> ZoneId {
        self.id
    }

    #[inline]
    pub fn parent_hyper(&self) -> HyperconnectorId {
        self.parent_hyper
    }

    #[inline]
    pub fn contained_entities(&self) -> &[EntityId] {
        &self.contained_entities
    }

    pub fn add_entity(&mut self, entity: EntityId) {
        if !self.contained_entities.contains(&entity) {
            self.contained_entities.push(entity);
        }
    }

    pub fn remove_entity(&mut self, entity: &EntityId) -> bool {
        if let Some(pos) = self.contained_entities.iter().position(|e| e == entity) {
            self.contained_entities.swap_remove(pos);
            true
        } else {
            false
        }
    }
}