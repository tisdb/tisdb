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