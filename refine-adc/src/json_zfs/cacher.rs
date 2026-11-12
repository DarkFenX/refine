use std::{
    fs::{File, create_dir_all},
    io::{self, BufReader},
    path::PathBuf,
};

use super::error::{JsonZfsAdcDataReadError, JsonZfsAdcFpReadError, JsonZfsAdcWriteError};
use crate::{VERSION, zfs_shared};

/// JSON adapted data cacher implementation.
///
/// This cacher implements persistent cache store in the form of zstd-compressed JSON.
pub struct JsonZfsAdc {
    dir: PathBuf,
    name: String,
}
impl JsonZfsAdc {
    /// Constructs new cacher using path to cache directory and cache file name (without extension).
    pub fn new(dir: impl Into<PathBuf>, name: impl Into<String>) -> Self {
        Self {
            dir: dir.into(),
            name: name.into(),
        }
    }
}
impl std::fmt::Debug for JsonZfsAdc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JsonZfsAdc(\"{}\")",
            self.get_cache_path().to_str().unwrap_or("<error>")
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cacher trait implementation
////////////////////////////////////////////////////////////////////////////////////////////////////
impl rc::ad::AdaptedDataCacherCore for JsonZfsAdc {
    fn load_from_cache(&self, fingerprint: &rc::ad::AFingerprint) -> Result<rc::ad::AData, rc::ad::err::AdcReadError> {
        let file = File::open(self.get_cache_path()).map_err(JsonZfsAdcFpReadError::Read)?;
        let mut reader = BufReader::new(file);
        let cached_fingerprint = zfs_shared::read_fp(&mut reader).map_err(JsonZfsAdcFpReadError::from)?;
        if cached_fingerprint != *fingerprint {
            return Err(rc::ad::err::AdcReadError::FingerprintMismatch(cached_fingerprint));
        }
        let decoder = zstd::stream::Decoder::with_buffer(reader).map_err(JsonZfsAdcDataReadError::Read)?;
        let a_data = serde_json::from_reader(BufReader::new(decoder)).map_err(JsonZfsAdcDataReadError::from)?;
        Ok(a_data)
    }
    fn write_cache(
        &self,
        a_data: &rc::ad::AData,
        fingerprint: rc::ad::AFingerprint,
    ) -> Result<(), rc::ad::err::AdcWriteError> {
        self.create_cache_dir()?;
        zfs_shared::write(&self.get_cache_path(), &fingerprint, |writer| {
            serde_json::to_writer(writer, a_data).map_err(JsonZfsAdcWriteError::from)
        })?;
        Ok(())
    }
    fn get_cacher_version(&self) -> String {
        VERSION.to_string()
    }
}

impl JsonZfsAdc {
    fn get_cache_path(&self) -> PathBuf {
        self.dir.join(format!("{}.json.zst", self.name))
    }
    fn create_cache_dir(&self) -> Result<(), JsonZfsAdcWriteError> {
        match create_dir_all(&self.dir) {
            Ok(()) => Ok(()),
            Err(e) => {
                match e.kind() {
                    // It's fine if it already exists for our purposes
                    io::ErrorKind::AlreadyExists => Ok(()),
                    _ => Err(JsonZfsAdcWriteError::CreateDir(e)),
                }
            }
        }
    }
}
