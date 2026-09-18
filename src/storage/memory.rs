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

    fn write_bytes(&mut self, offset: u64, bytes: &[u8]) -> Result<(), CoreError> {
        let start = offset as usize;
        let end = start + bytes.len();
        if end > self.data.len() {
            self.data.resize(end, 0);
        }
        self.data[start..end].copy_from_slice(bytes);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), CoreError> {
        Ok(())
    }

    fn len(&self) -> u64 {
        self.data.len() as u64
    }
}