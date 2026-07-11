use std::{
    fmt,
    fs::{OpenOptions, create_dir_all},
    io::{self, Write},
    path::PathBuf,
};

use super::{
    data::CData,
    error::{JsonZfileAdcDataReadError, JsonZfileAdcFpReadError, JsonZfileAdcWriteError},
};
use crate::VERSION;

/// JSON adapted data cacher implementation.
///
/// This cacher implements persistent cache store in the form of zstd-compressed JSON.
pub struct JsonZfileAdc {
    folder: PathBuf,
    name: String,
}
impl JsonZfileAdc {
    /// Constructs new cacher using path to cache folder and cache file name (without extension).
    pub fn new(folder: PathBuf, name: String) -> Self {
        Self { folder, name }
    }
    fn get_cache_path(&self) -> PathBuf {
        self.folder.join(format!("{}.json.zst", self.name))
    }
    fn get_fingerprint_path(&self) -> PathBuf {
        self.folder.join(format!("{}_fp.txt", self.name))
    }
    fn create_cache_folder(&self) -> Result<(), JsonZfileAdcWriteError> {
        match create_dir_all(&self.folder) {
            Ok(()) => Ok(()),
            Err(e) => {
                match e.kind() {
                    // It's fine if it already exists for our purposes
                    io::ErrorKind::AlreadyExists => Ok(()),
                    _ => Err(JsonZfileAdcWriteError::CreateFolderFailed(e.to_string())),
                }
            }
        }
    }
    fn write_data(&self, c_data: CData) -> Result<(), JsonZfileAdcWriteError> {
        let cache_path = self.get_cache_path();
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(cache_path)?;
        let writer = zstd::stream::Encoder::new(file, 7)?.auto_finish();
        c_data.try_serialize(writer)?;
        Ok(())
    }
    fn write_fingerprint(&self, fingerprint: rc::ad::AFingerprint) -> Result<(), JsonZfileAdcWriteError> {
        let fp_path = self.get_fingerprint_path();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(fp_path)
            .map_err(|e| JsonZfileAdcWriteError::FpWriteFailed(e.to_string()))?;
        write!(file, "{fingerprint}").map_err(|e| JsonZfileAdcWriteError::FpWriteFailed(e.to_string()))?;
        Ok(())
    }
}
impl fmt::Debug for JsonZfileAdc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "JsonZfileAdc(\"{}\")",
            self.get_cache_path().to_str().unwrap_or("<error>")
        )
    }
}
impl rc::ad::AdaptedDataCacher for JsonZfileAdc {
    fn get_cache_fingerprint(&mut self) -> Result<rc::ad::AFingerprint, Box<dyn std::error::Error>> {
        let fingerprint = std::fs::read_to_string(self.get_fingerprint_path())
            .map_err(|e| JsonZfileAdcFpReadError::ReadFailed(e.to_string()))?;
        Ok(rc::ad::AFingerprint::from_string(fingerprint.trim().into()))
    }
    fn load_from_cache(&mut self) -> Result<rc::ad::AData, Box<dyn std::error::Error>> {
        let full_path = self.get_cache_path();
        let file = OpenOptions::new()
            .read(true)
            .open(full_path)
            .map_err(|e| JsonZfileAdcDataReadError::ReadFailed(e.to_string()))?;
        let reader =
            zstd::stream::Decoder::new(file).map_err(|e| JsonZfileAdcDataReadError::ReadFailed(e.to_string()))?;
        let c_data = CData::try_deserialize(reader)?;
        Ok(c_data.into_adapted())
    }
    fn write_cache(
        &mut self,
        a_data: &rc::ad::AData,
        fingerprint: rc::ad::AFingerprint,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.create_cache_folder()?;
        self.write_data(CData::from_adapted(a_data))?;
        self.write_fingerprint(fingerprint)?;
        Ok(())
    }
    fn get_cacher_version(&self) -> String {
        VERSION.to_string()
    }
}
