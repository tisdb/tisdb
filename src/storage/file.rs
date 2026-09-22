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