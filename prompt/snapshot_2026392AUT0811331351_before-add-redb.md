# CODE SNAPSHOT v:2026392AUT0811331351

## Metadata

```text
├─ Katalog roboczy (CWD): A:/A-JAN/WIN-DOCS/REPO_OWN_RUST/GIT_tisdb/tisdb
├─ Lokalizacje (scan_at): ["./"]
└─ Wzorce (match_pattern): ["./{examples|tests|docs|src}/**", "./{Cargo.toml|README.md}"]
📦 Zeskanowano fizycznie: 400 plików, 118 katalogów
```

## Structure

```plaintext
  ▣─┬ tisdb                             [459.7 KiB]                      A:/A-JAN/WIN-DOCS/REPO_OWN_RUST/GIT 
    │                                                                    _tisdb/tisdb/                       
 1  ├──• Cargo.toml                     [   1018 B] [2026-39-2 07:58:03] ./Cargo.toml                        
    ├──┬ docs                           [417.7 KiB] [2026-39-2 07:45:33] ./docs/                             
    │  └──┬ images                      [417.7 KiB] [2026-39-2 07:45:33] ./docs/images/                      
    │     └──• GRAFY_CISOWSKIEGO.png    [417.7 KiB] [2026-39-2 07:45:33] ./docs/images/GRAFY_CISOWSKIEGO.png 
    ├──┬ src                            [ 31.7 KiB] [2026-39-2 07:47:10] ./src/                              
 2  │  ├──• engine.rs                   [ 12.8 KiB] [2026-39-1 23:08:51] ./src/engine.rs                     
 3  │  ├──• error.rs                    [  1.1 KiB] [2026-39-1 22:33:50] ./src/error.rs                      
 4  │  ├──• lib.rs                      [    770 B] [2026-39-1 21:54:27] ./src/lib.rs                        
    │  ├──┬ domain                      [ 11.6 KiB] [2026-39-2 07:47:10] ./src/domain/                       
 5  │  │  ├──• flow.rs                  [  1.8 KiB] [2026-39-1 21:54:01] ./src/domain/flow.rs                
 6  │  │  ├──• hyper.rs                 [  2.0 KiB] [2026-39-1 21:52:34] ./src/domain/hyper.rs               
 7  │  │  ├──• id.rs                    [  1.9 KiB] [2026-37-3 13:41:25] ./src/domain/id.rs                  
 8  │  │  ├──• metadata.rs              [  3.6 KiB] [2026-39-1 22:48:17] ./src/domain/metadata.rs            
 9  │  │  ├──• node.rs                  [    701 B] [2026-39-1 21:52:19] ./src/domain/node.rs                
10  │  │  └──• zone.rs                  [  1.5 KiB] [2026-39-1 21:53:46] ./src/domain/zone.rs                
    │  └──┬ storage                     [  5.4 KiB] [2026-39-2 07:47:10] ./src/storage/                      
11  │     ├──• backend.rs               [    610 B] [2026-39-1 23:07:07] ./src/storage/backend.rs            
12  │     ├──• file.rs                  [  3.7 KiB] [2026-39-1 23:07:44] ./src/storage/file.rs               
13  │     └──• memory.rs                [  1.1 KiB] [2026-39-1 23:08:08] ./src/storage/memory.rs             
    └──┬ tests                          [  9.3 KiB] [2026-39-2 07:47:10] ./tests/                            
14     ├──• axiom_tests.rs              [  1.9 KiB] [2026-39-1 21:55:27] ./tests/axiom_tests.rs              
15     ├──• metadata_tests.rs           [  1.1 KiB] [2026-39-1 21:56:23] ./tests/metadata_tests.rs           
16     ├──• persistence_tests.rs        [  4.6 KiB] [2026-39-1 22:49:39] ./tests/persistence_tests.rs        
17     └──• storage_tests.rs            [  1.6 KiB] [2026-39-1 22:50:01] ./tests/storage_tests.rs            
```

## Source Code Content

### [1] `./Cargo.toml`

```toml
[package]
name = "tisdb"
version = "0.1.0-dev.1"
authors = ["Jan Roman Cisowski „j-Cis” <code@cisowscy.com>"]
license = "MIT OR Apache-2.0"
edition = "2024"
rust-version = "1.98.0"
repository = "https://github.com/tisdb/tisdb"
resolver = "3"


[package.metadata.cargo]
edition = "2024"

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
targets = [
    "x86_64-pc-windows-msvc",
    "i686-pc-windows-msvc",
    "x86_64-unknown-linux-gnu",
    "x86_64-unknown-linux-musl",
    "i686-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin"
]

[lib]
crate-type = ["cdylib", "rlib"]

[badges]
travis-ci = { repository = "tisdb/tisdb" }
appveyor = { repository = "tisdb/tisdb" }

[dependencies]
fslock = "0.2.1"
rkyv = "0.8.18"
serde = { version = "1.0.229", features = ["derive"] }
thiserror = "2.0.20"
ulid = "3.0.0"
wasm-bindgen = { version = "0.2.128", optional = true }

[features]
wasm-bindgen = ["dep:wasm-bindgen"]


```

### [2] `./src/domain/flow.rs`

```rust
use crate::domain::id::{FlowId, HyperconnectorId, ZoneId};
use crate::domain::metadata::EntityHeader;
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
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq,
)]
#[rkyv(derive(Debug, PartialEq))]
pub struct Flow {
    id: FlowId,
    parent_hyper: HyperconnectorId,
    source_zone: ZoneId,
    target_zone: ZoneId,
    flow_type: FlowType,
    pub header: EntityHeader,
}

impl Flow {
    pub fn new(
        parent_hyper: HyperconnectorId,
        source_zone: ZoneId,
        target_zone: ZoneId,
        flow_type: FlowType,
        class_path: Vec<String>,
    ) -> Self {
        Self {
            id: FlowId::new(),
            parent_hyper,
            source_zone,
            target_zone,
            flow_type,
            header: EntityHeader::new(class_path),
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
```

### [3] `./src/domain/hyper.rs`

```rust
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
```

### [4] `./src/domain/id.rs`

```rust
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use ulid::Ulid;

macro_rules! define_id {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
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
            PartialOrd,
            Ord,
        )]
        #[rkyv(derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord))]
        pub struct $name([u8; 16]);

        impl $name {
    #[inline]
    pub fn new() -> Self {
        Self(Ulid::generate().to_bytes())
    }

            #[inline]
            pub fn from_bytes(bytes: [u8; 16]) -> Self {
                Self(bytes)
            }

            #[inline]
            pub fn as_bytes(&self) -> &[u8; 16] {
                &self.0
            }
        }

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self::new()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", Ulid::from(self.0))
            }
        }
    };
}

define_id!(NodeId, "Unikalne ID atomowego Węzła (N)");
define_id!(HyperconnectorId, "Unikalne ID Hiperkonektora (H)");
define_id!(ZoneId, "Unikalne ID Strefy / Portu (Z)");
define_id!(FlowId, "Unikalne ID Przepływu (F)");

/// Uniwersum Obiektów U = N ⊔ H (Aksjomat 2)
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
pub enum EntityId {
    Node(NodeId),
    Hyperconnector(HyperconnectorId),
}
```

### [5] `./src/domain/metadata.rs`

```rust
use std::collections::HashMap;

use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

/// Definiuje format tekstu dla zawartości RichText.
#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq, Eq, Hash,
)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub enum TextFormat {
    Plain,
    Markdown,
    Html,
    Json,
    Xml,
}

/// Skalarny typ pomocniczy dla struktur złożonych (zbiory, przedziały), 
/// zapobiegający rekurencji nieskończonej.
#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq,
)]
#[rkyv(derive(Debug, PartialEq))]
pub enum ScalarValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

/// Główny, w pełni elastyczny typ atrybutów dla grafu.
#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq,
)]
#[rkyv(derive(Debug, PartialEq))]
pub enum AttributeValue {
    // --- SKALARNE ---
    ShortText(String),
    LongText(String),
    RichText { format: TextFormat, content: String },
    Integer(i64),
    Float(f64),
    Boolean(bool),

    // --- ZŁOŻONE ---
    /// Zbiór wartości skalarnych (np. Tagi).
    Set(Vec<ScalarValue>),
    /// Uporządkowana lista/sekwencja.
    Sequence(Vec<ScalarValue>),
    /// Zakres/Przedział min-max.
    Range { min: ScalarValue, max: ScalarValue },
}

/// Uniwersalny nagłówek (Entity Header) przypinany do każdego obiektu w systemie (N, H, Z, F).
#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq,
)]
#[rkyv(derive(Debug, PartialEq))]
pub struct EntityHeader {
    // 1. Właściwości Systemowe (Audyt)
    created_at: u64,
    updated_at: u64,
    revision: u32,

    // 2. Ontologia (Ścieżka klasyfikacji, np. ["Osoba", "Kobieta"])
    class_path: Vec<String>,

    // 3. Właściwości Dynamiczne - HashMap (rkyv obsługuje to doskonale, a dla nas O(1))
    attributes: HashMap<String, AttributeValue>,
}

impl EntityHeader {
    pub fn new(class_path: Vec<String>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            created_at: now,
            updated_at: now,
            revision: 1,
            class_path,
            attributes: HashMap::new(),
        }
    }

    // --- Systemowe gettery ---
    pub fn created_at(&self) -> u64 { self.created_at }
    pub fn updated_at(&self) -> u64 { self.updated_at }
    pub fn revision(&self) -> u32 { self.revision }
    pub fn class_path(&self) -> &[String] { &self.class_path }

    // --- Modulatory atrybutów ---
    pub fn set_attribute(&mut self, key: impl Into<String>, value: AttributeValue) {
        self.attributes.insert(key.into(), value);
        self.bump_revision();
    }

    pub fn get_attribute(&self, key: &str) -> Option<&AttributeValue> {
        self.attributes.get(key)
    }

    pub fn remove_attribute(&mut self, key: &str) -> Option<AttributeValue> {
        let removed = self.attributes.remove(key);
        if removed.is_some() {
            self.bump_revision();
        }
        removed
    }

    // --- Prywatne podbicie wersji ---
    fn bump_revision(&mut self) {
        self.revision += 1;
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }
}
```

### [6] `./src/domain/node.rs`

```rust
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
```

### [7] `./src/domain/zone.rs`

```rust
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
```

### [8] `./src/engine.rs`

```rust
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
```

### [9] `./src/error.rs`

```rust
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CoreError {
    #[error("Obiekt o podanym ID nie istnieje: {0}")]
    NotFound(String),

    #[error("Naruszenie Aksjomatu 3: Strefa należy już do innego hiperkonektora")]
    Axiom3Violation,

    #[error("Naruszenie Aksjomatu 5: Przepływ należy już do innego hiperkonektora")]
    Axiom5Violation,

    #[error("Naruszenie Aksjomatu 9: Wykryto cykl zagnieżdżenia lub cykl cykliczny w relacji R_contain")]
    Axiom9CycleDetected,

    #[error("Naruszenie Aksjomatu 10: Przepływ próbuje łączyć strefy z różnych hiperkonektorów")]
    InvalidFlowBoundary,

    #[error("Błąd warstwy pamięci masowej (IO/Storage): {0}")]
    StorageError(String),

    // --- ZMIANA: Dodano pole (String) aby przenosić komunikat błędu rancor ---
    #[error("Błąd binarnej serializacji/deserializacji (rkyv): {0}")]
    SerializationError(String),

    // --- DODANE BŁĘDY RESTRICT/GUARDRAILS ---
    #[error("Obiekt {0} jest używany i nie może zostać usunięty: {1}")]
    InUse(String, String),
}
```

### [10] `./src/lib.rs`

```rust
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
```

### [11] `./src/storage/backend.rs`

```rust
// src/storage/backend.rs
use crate::error::CoreError;

pub trait StorageBackend {
    /// Czyta bajty z trwałego nośnika
    fn read_bytes(&self, offset: u64, len: usize) -> Result<Vec<u8>, CoreError>;
    
    /// Wykonuje w pełni atomowy zapis całego zrzutu pamięci.
    /// Jeśli operacja się nie powiedzie, stary stan bazy pozostaje nietknięty.
    fn atomic_write_snapshot(&mut self, payload: &[u8]) -> Result<(), CoreError>;
    
    /// Zwraca wielkość obecnego zrzutu
    fn len(&self) -> u64;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
```

### [12] `./src/storage/file.rs`

```rust
// src/storage/file.rs
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use fslock::LockFile;

use crate::error::CoreError;
use crate::storage::backend::StorageBackend;

pub struct FileStorage {
    path: PathBuf,
    file: Option<File>, // Opcjonalny, aby można było go bezpiecznie zamknąć przy podmianie pliku na Windowsie
    _lock: LockFile,
}

impl FileStorage {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, CoreError> {
        let db_path = path.as_ref().to_path_buf();
        let lock_path = db_path.with_extension("cdb.lock");
        
        let mut lock = LockFile::open(&lock_path)
            .map_err(|e| CoreError::StorageError(format!("Nie można utworzyć pliku blokady: {e}")))?;

        if !lock
            .try_lock()
            .map_err(|e| CoreError::StorageError(format!("Błąd prób blokady pliku: {e}")))?
        {
            return Err(CoreError::StorageError(
                "Plik bazy jest używany przez inny proces".into(),
            ));
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&db_path)
            .map_err(|e| CoreError::StorageError(format!("Błąd otwarcia pliku bazy: {e}")))?;

        Ok(Self { 
            path: db_path, 
            file: Some(file), 
            _lock: lock 
        })
    }
}

impl StorageBackend for FileStorage {
    fn read_bytes(&self, offset: u64, len: usize) -> Result<Vec<u8>, CoreError> {
        let mut file = self.file.as_ref()
            .ok_or_else(|| CoreError::StorageError("Uchwyt pliku jest zamknięty".into()))?;
            
        file.seek(SeekFrom::Start(offset))
            .map_err(|e| CoreError::StorageError(e.to_string()))?;

        let mut buffer = vec![0u8; len];
        file.read_exact(&mut buffer)
            .map_err(|e| CoreError::StorageError(e.to_string()))?;

        Ok(buffer)
    }

    fn atomic_write_snapshot(&mut self, payload: &[u8]) -> Result<(), CoreError> {
        let temp_path = self.path.with_extension("cdb.tmp");

        // 1. Otwórz tymczasowy plik i zapisz dane
        let mut temp_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_path)
            .map_err(|e| CoreError::StorageError(format!("Błąd tworzenia pliku tmp: {e}")))?;

        temp_file.write_all(payload)
            .map_err(|e| CoreError::StorageError(format!("Błąd zapisu do pliku tmp: {e}")))?;
            
        // 2. Wymuś zrzut z buforów OS fizycznie na dysk
        temp_file.sync_all()
            .map_err(|e| CoreError::StorageError(format!("Błąd synchronizacji I/O dysku: {e}")))?;

        // 3. Zamknij stary uchwyt (niezbędne na Windowsie przed zrobieniem atomowego rename)
        self.file = None;

        // 4. Atomowa podmiana pliku (na platformach POSIX i nowoczesnym NTFS)
        std::fs::rename(&temp_path, &self.path)
            .map_err(|e| CoreError::StorageError(format!("Błąd atomowej podmiany pliku bazy: {e}")))?;

        // 5. Otwórz nowy plik i przypisz do struktury aby przywrócić możliwość czytania
        let new_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .map_err(|e| CoreError::StorageError(format!("Błąd ponownego otwarcia zaktualizowanego pliku: {e}")))?;

        self.file = Some(new_file);

        Ok(())
    }

    fn len(&self) -> u64 {
        if let Some(f) = &self.file {
            f.metadata().map(|m| m.len()).unwrap_or(0)
        } else {
            0
        }
    }
}
```

### [13] `./src/storage/memory.rs`

```rust
// src/storage/memory.rs
use crate::error::CoreError;
use crate::storage::backend::StorageBackend;

#[derive(Debug, Default, Clone)]
pub struct MemoryStorage {
    data: Vec<u8>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { data: bytes }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl StorageBackend for MemoryStorage {
    fn read_bytes(&self, offset: u64, len: usize) -> Result<Vec<u8>, CoreError> {
        let start = offset as usize;
        let end = start + len;
        if end > self.data.len() {
            return Err(CoreError::StorageError("Odczyt poza zakresem pamięci".into()));
        }
        Ok(self.data[start..end].to_vec())
    }

    fn atomic_write_snapshot(&mut self, payload: &[u8]) -> Result<(), CoreError> {
        // W pamięci RAM "atomowa" podmiana to po prostu nadpisanie wektora sklonowanymi bajtami
        self.data = payload.to_vec();
        Ok(())
    }

    fn len(&self) -> u64 {
        self.data.len() as u64
    }
}
```

### [14] `./tests/axiom_tests.rs`

```rust
use tisdb::{CisowskiEngine, CoreError, EntityId, FlowType, MemoryStorage};

#[test]
fn test_node_and_hyperconnector_creation() {
    let storage = MemoryStorage::new();
    let mut engine = CisowskiEngine::new(storage);

    let node_id = engine.create_node(vec!["TestNode".into()]);
    let hyper_id = engine.create_hyperconnector(vec!["TestHyper".into()]);

    assert!(engine.get_node(&node_id).is_some());
    assert!(engine.get_hyperconnector(&hyper_id).is_some());
}

#[test]
fn test_axiom_3_and_4_zone_creation() {
    let storage = MemoryStorage::new();
    let mut engine = CisowskiEngine::new(storage);

    let hyper_id = engine.create_hyperconnector(vec![]);
    let zone_id = engine.create_zone(hyper_id, vec![]).unwrap();

    let zone = engine.get_zone(&zone_id).unwrap();
    assert_eq!(zone.parent_hyper(), hyper_id);
}

#[test]
fn test_axiom_9_cycle_prevention() {
    let storage = MemoryStorage::new();
    let mut engine = CisowskiEngine::new(storage);

    let h1 = engine.create_hyperconnector(vec![]);
    let z1 = engine.create_zone(h1, vec![]).unwrap();

    let h2 = engine.create_hyperconnector(vec![]);
    let z2 = engine.create_zone(h2, vec![]).unwrap();

    engine
        .add_entity_to_zone(z1, EntityId::Hyperconnector(h2))
        .unwrap();

    let result = engine.add_entity_to_zone(z2, EntityId::Hyperconnector(h1));
    assert_eq!(result, Err(CoreError::Axiom9CycleDetected));
}

#[test]
fn test_axiom_10_flow_boundaries() {
    let storage = MemoryStorage::new();
    let mut engine = CisowskiEngine::new(storage);

    let h1 = engine.create_hyperconnector(vec![]);
    let z1 = engine.create_zone(h1, vec![]).unwrap();

    let h2 = engine.create_hyperconnector(vec![]);
    let z2 = engine.create_zone(h2, vec![]).unwrap();

    let result = engine.create_flow(h1, z1, z2, FlowType::Dir, vec![]);
    assert_eq!(result, Err(CoreError::InvalidFlowBoundary));
}
```

### [15] `./tests/metadata_tests.rs`

```rust
use tisdb::{AttributeValue, CisowskiEngine, MemoryStorage};

#[test]
fn test_metadata_attributes_lifecycle() {
    let storage = MemoryStorage::new();
    let mut engine = CisowskiEngine::new(storage);

    // 1. Tworzenie węzła z zadeklarowaną klasą
    let nid = engine.create_node(vec!["Osoba".into(), "Mężczyzna".into()]);
    
    // 2. Pobranie i modyfikacja atrybutów (Update)
    let node = engine.get_node_mut(&nid).unwrap();
    assert_eq!(node.header.revision(), 1); // Wersja po utworzeniu

    node.header.set_attribute("Wiek", AttributeValue::Integer(35));
    node.header.set_attribute("Aktywny", AttributeValue::Boolean(true));

    assert_eq!(node.header.revision(), 3); // Wersja urosła dwukrotnie po dwóch update'ach

    // 3. Sprawdzenie, czy dane przetrwały i są prawidłowego typu
    let fetched_node = engine.get_node(&nid).unwrap();
    assert_eq!(fetched_node.header.class_path(), &["Osoba", "Mężczyzna"]);
    
    match fetched_node.header.get_attribute("Wiek") {
        Some(AttributeValue::Integer(val)) => assert_eq!(*val, 35),
        _ => panic!("Nieprawidłowy typ lub brak atrybutu Wiek"),
    }
}
```

### [16] `./tests/persistence_tests.rs`

```rust
use std::fs;
use tisdb::{AttributeValue, CisowskiEngine, EntityId, FileStorage, ScalarValue, TextFormat};

#[test]
fn test_binary_save_and_load_cycle() {
    let db_path = "test_persistence.cdb";
    let lock_path = "test_persistence.cdb.lock";

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    let (node_id, hyper_id, zone_id) = {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);

        let nid = engine.create_node(vec!["PersistNode".into()]);
        let hid = engine.create_hyperconnector(vec!["PersistHyper".into()]);
        let zid = engine.create_zone(hid, vec![]).unwrap();

        engine
            .add_entity_to_zone(zid, EntityId::Node(nid))
            .unwrap();

        engine.save().expect("Błąd podczas zapisu migawki");
        (nid, hid, zid)
    };

    // Ponowne otwarcie tej samej bazy z dysku
    {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);
        engine.load().expect("Błąd podczas odczytu migawki");

        assert!(engine.get_node(&node_id).is_some());
        assert!(engine.get_hyperconnector(&hyper_id).is_some());

        let zone = engine.get_zone(&zone_id).unwrap();
        assert_eq!(zone.contained_entities().len(), 1);
        assert_eq!(zone.contained_entities()[0], EntityId::Node(node_id));
        
        let node = engine.get_node(&node_id).unwrap();
        assert_eq!(node.header.class_path()[0], "PersistNode");
    }

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}

#[test]
fn test_truncate_on_shrink() {
    let db_path = "test_truncate.cdb";
    let lock_path = "test_truncate.cdb.lock";
    
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    // KROK 1: Zapis dużej bazy
    let size_large = {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);
        for _ in 0..100 {
             engine.create_node(vec![]);
        }
        engine.save().unwrap();
        fs::metadata(db_path).unwrap().len()
    };
    
    // KROK 2: Nadpisanie małą bazą
    let size_small = {
        let storage = FileStorage::open(db_path).unwrap(); // Otwiera istniejacy plik
        let mut engine = CisowskiEngine::new(storage);
        // Nadpisuje 100 węzłów z pamięci pustym grafem
        engine.save().unwrap();
        fs::metadata(db_path).unwrap().len()
    };
    
    assert!(size_small < size_large, "Plik powinien zostać skrócony (truncated)");
    
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}

#[test]
fn test_attributes_persistence_hashmap() {
    let db_path = "test_attrs.cdb";
    let lock_path = "test_attrs.cdb.lock";
    
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    let node_id = {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);
        let nid = engine.create_node(vec![]);
        
        let node = engine.get_node_mut(&nid).unwrap();
        node.header.set_attribute("wiek", AttributeValue::Integer(99));
        node.header.set_attribute("opis", AttributeValue::RichText { 
            format: TextFormat::Markdown, 
            content: "**Test**".to_string() 
        });
        node.header.set_attribute("tagi", AttributeValue::Set(vec![
             ScalarValue::String("tag1".into()),
             ScalarValue::String("tag2".into()),
        ]));
        
        engine.save().unwrap();
        nid
    };

    {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);
        engine.load().unwrap();
        
        let node = engine.get_node(&node_id).unwrap();
        assert_eq!(node.header.get_attribute("wiek"), Some(&AttributeValue::Integer(99)));
        
        match node.header.get_attribute("opis") {
            Some(AttributeValue::RichText { format, content }) => {
                assert_eq!(format, &TextFormat::Markdown);
                assert_eq!(content, "**Test**");
            },
            _ => panic!("Błąd persystencji RichText"),
        }
        
        match node.header.get_attribute("tagi") {
            Some(AttributeValue::Set(vec)) => {
                assert_eq!(vec.len(), 2);
            },
            _ => panic!("Błąd persystencji Set"),
        }
    }
    
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}
```

### [17] `./tests/storage_tests.rs`

```rust
use std::fs::{self, OpenOptions};
use std::io::Write;

use tisdb::{CisowskiEngine, CoreError, FileStorage};

#[test]
fn test_file_storage_lifecycle() {
    let db_path = "test_graph.cdb";
    let lock_path = "test_graph.cdb.lock";

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    {
        let storage = FileStorage::open(db_path).expect("Nie udało się utworzyć magazynu plików");
        let mut engine = CisowskiEngine::new(storage);

        let _node_id = engine.create_node(vec![]);
        let _hyper_id = engine.create_hyperconnector(vec![]);
    }

    assert!(fs::metadata(db_path).is_ok());

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}

#[test]
fn test_broken_header() {
    let db_path = "test_broken.cdb";
    let lock_path = "test_broken.cdb.lock";
    
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    // Tworzymy "uszkodzony" plik
    {
        let mut file = OpenOptions::new().write(true).create(true).open(db_path).unwrap();
        file.write_all(b"CDB").unwrap(); // Tylko 3 bajty, za mało na nagłówek
    }

    {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);
        let result = engine.load();
        
        match result {
             Err(CoreError::StorageError(msg)) => assert!(msg.contains("ucięty nagłówek")),
             _ => panic!("System nie wykrył uszkodzonego nagłówka"),
        }
    }

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}
```

## End Structure Summary

```plaintext
  ▣─┬ tisdb                             [459.7 KiB]                      A:/A-JAN/WIN-DOCS/REPO_OWN_RUST/GIT 
    │                                                                    _tisdb/tisdb/                       
 1  ├──• Cargo.toml                     [   1018 B] [2026-39-2 07:58:03] ./Cargo.toml                        
    ├──┬ docs                           [417.7 KiB] [2026-39-2 07:45:33] ./docs/                             
    │  └──┬ images                      [417.7 KiB] [2026-39-2 07:45:33] ./docs/images/                      
    │     └──• GRAFY_CISOWSKIEGO.png    [417.7 KiB] [2026-39-2 07:45:33] ./docs/images/GRAFY_CISOWSKIEGO.png 
    ├──┬ src                            [ 31.7 KiB] [2026-39-2 07:47:10] ./src/                              
 2  │  ├──• engine.rs                   [ 12.8 KiB] [2026-39-1 23:08:51] ./src/engine.rs                     
 3  │  ├──• error.rs                    [  1.1 KiB] [2026-39-1 22:33:50] ./src/error.rs                      
 4  │  ├──• lib.rs                      [    770 B] [2026-39-1 21:54:27] ./src/lib.rs                        
    │  ├──┬ domain                      [ 11.6 KiB] [2026-39-2 07:47:10] ./src/domain/                       
 5  │  │  ├──• flow.rs                  [  1.8 KiB] [2026-39-1 21:54:01] ./src/domain/flow.rs                
 6  │  │  ├──• hyper.rs                 [  2.0 KiB] [2026-39-1 21:52:34] ./src/domain/hyper.rs               
 7  │  │  ├──• id.rs                    [  1.9 KiB] [2026-37-3 13:41:25] ./src/domain/id.rs                  
 8  │  │  ├──• metadata.rs              [  3.6 KiB] [2026-39-1 22:48:17] ./src/domain/metadata.rs            
 9  │  │  ├──• node.rs                  [    701 B] [2026-39-1 21:52:19] ./src/domain/node.rs                
10  │  │  └──• zone.rs                  [  1.5 KiB] [2026-39-1 21:53:46] ./src/domain/zone.rs                
    │  └──┬ storage                     [  5.4 KiB] [2026-39-2 07:47:10] ./src/storage/                      
11  │     ├──• backend.rs               [    610 B] [2026-39-1 23:07:07] ./src/storage/backend.rs            
12  │     ├──• file.rs                  [  3.7 KiB] [2026-39-1 23:07:44] ./src/storage/file.rs               
13  │     └──• memory.rs                [  1.1 KiB] [2026-39-1 23:08:08] ./src/storage/memory.rs             
    └──┬ tests                          [  9.3 KiB] [2026-39-2 07:47:10] ./tests/                            
14     ├──• axiom_tests.rs              [  1.9 KiB] [2026-39-1 21:55:27] ./tests/axiom_tests.rs              
15     ├──• metadata_tests.rs           [  1.1 KiB] [2026-39-1 21:56:23] ./tests/metadata_tests.rs           
16     ├──• persistence_tests.rs        [  4.6 KiB] [2026-39-1 22:49:39] ./tests/persistence_tests.rs        
17     └──• storage_tests.rs            [  1.6 KiB] [2026-39-1 22:50:01] ./tests/storage_tests.rs            
```

---
*Generated automatically by querypath-snapshot*
