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