use std::{
    fs::{OpenOptions, create_dir_all},
    io::{self, BufReader, BufWriter, Write},
    path::PathBuf,
};

use super::error::{JsonZfsAdcDataReadError, JsonZfsAdcFpReadError, JsonZfsAdcWriteError};
use crate::VERSION;

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
impl rc::ad::AdaptedDataCacherInterface for JsonZfsAdc {
    fn get_cache_fingerprint(&self) -> Result<rc::ad::AFingerprint, rc::ad::err::AdaptedDataCacherError> {
        let fingerprint = std::fs::read_to_string(self.get_fingerprint_path()).map_err(JsonZfsAdcFpReadError::Read)?;
        Ok(rc::ad::AFingerprint::from_string(fingerprint.trim().into()))
    }
    fn load_from_cache(&self) -> Result<rc::ad::AData, rc::ad::err::AdaptedDataCacherError> {
        let full_path = self.get_cache_path();
        let file = OpenOptions::new()
            .read(true)
            .open(full_path)
            .map_err(JsonZfsAdcDataReadError::Read)?;
        let reader = zstd::stream::Decoder::new(file).map_err(JsonZfsAdcDataReadError::Read)?;
        let a_data = serde_json::from_reader(BufReader::new(reader)).map_err(JsonZfsAdcDataReadError::from)?;
        Ok(a_data)
    }
    fn write_cache(
        &self,
        a_data: &rc::ad::AData,
        fingerprint: rc::ad::AFingerprint,
    ) -> Result<(), rc::ad::err::AdaptedDataCacherError> {
        self.create_cache_dir()?;
        self.write_data(a_data)?;
        self.write_fingerprint(fingerprint)?;
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
    fn get_fingerprint_path(&self) -> PathBuf {
        self.dir.join(format!("{}.json.zst.fp", self.name))
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
    fn write_data(&self, a_data: &rc::ad::AData) -> Result<(), JsonZfsAdcWriteError> {
        let cache_path = self.get_cache_path();
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(cache_path)?;
        // zstd has internal input buffer, but it uses FFI which makes it moderately expensive;
        // serde-json writes very few bytes at a time, so native write buffer actually helps
        let writer = BufWriter::new(zstd::stream::Encoder::new(file, 7)?.auto_finish());
        serde_json::to_writer(writer, a_data)?;
        Ok(())
    }
    fn write_fingerprint(&self, fingerprint: rc::ad::AFingerprint) -> Result<(), JsonZfsAdcWriteError> {
        let fp_path = self.get_fingerprint_path();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(fp_path)
            .map_err(JsonZfsAdcWriteError::FpWrite)?;
        write!(file, "{fingerprint}").map_err(JsonZfsAdcWriteError::FpWrite)?;
        Ok(())
    }
}
