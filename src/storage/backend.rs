// src/storage/backend.rs
use crate::error::CoreError;

pub trait StorageBackend {
    fn read_bytes(&self, offset: u64, len: usize) -> Result<Vec<u8>, CoreError>;
    fn write_bytes(&mut self, offset: u64, bytes: &[u8]) -> Result<(), CoreError>;
    fn flush(&mut self) -> Result<(), CoreError>;
    fn len(&self) -> u64;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}