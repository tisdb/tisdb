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

    #[error("Błąd binarnej serializacji/deserializacji (rkyv)")]
    SerializationError,
}