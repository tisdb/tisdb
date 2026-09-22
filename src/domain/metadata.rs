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