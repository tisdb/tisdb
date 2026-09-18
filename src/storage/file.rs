use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use fslock::LockFile;

use crate::error::CoreError;
use crate::storage::backend::StorageBackend;

pub struct FileStorage {
    file: File,
    _lock: LockFile,
}

impl FileStorage {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, CoreError> {
        let lock_path = path.as_ref().with_extension("cdb.lock");
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
            .open(path)
            .map_err(|e| CoreError::StorageError(format!("Błąd otwarcia pliku bazy: {e}")))?;

        Ok(Self { file, _lock: lock })
    }
}

impl StorageBackend for FileStorage {
    fn read_bytes(&self, offset: u64, len: usize) -> Result<Vec<u8>, CoreError> {
        let mut file = &self.file;
        file.seek(SeekFrom::Start(offset))
            .map_err(|e| CoreError::StorageError(e.to_string()))?;

        let mut buffer = vec![0u8; len];
        file.read_exact(&mut buffer)
            .map_err(|e| CoreError::StorageError(e.to_string()))?;

        Ok(buffer)
    }

    fn write_bytes(&mut self, offset: u64, bytes: &[u8]) -> Result<(), CoreError> {
        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(|e| CoreError::StorageError(e.to_string()))?;

        self.file
            .write_all(bytes)
            .map_err(|e| CoreError::StorageError(e.to_string()))?;

        Ok(())
    }

    fn flush(&mut self) -> Result<(), CoreError> {
        self.file
            .sync_all()
            .map_err(|e| CoreError::StorageError(e.to_string()))
    }

    fn len(&self) -> u64 {
        self.file.metadata().map(|m| m.len()).unwrap_or(0)
    }
}