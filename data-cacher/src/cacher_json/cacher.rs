use std::{
    fmt,
    fs::{OpenOptions, create_dir_all},
    io::{self, Write},
    path::PathBuf,
};

use super::{data::CData, error::JsonZfileAdcError};
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
    fn create_cache_folder(&self) -> Option<String> {
        match create_dir_all(&self.folder) {
            Ok(_) => None,
            Err(e) => {
                match e.kind() {
                    // It's fine if it already exists for our purposes
                    io::ErrorKind::AlreadyExists => None,
                    _ => Some(e.to_string()),
                }
            }
        }
    }
    fn write_data(&self, c_data: CData) {
        let cache_path = self.get_cache_path();
        let file = match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(cache_path)
        {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("unable to open cache file for writing: {e}");
                return;
            }
        };
        let json = serde_json::json!(c_data).to_string();
        match zstd::stream::copy_encode(json.as_bytes(), file, 7) {
            Ok(_) => (),
            Err(e) => {
                tracing::error!("unable to write cache file: {e}")
            }
        };
    }
    fn write_fingerprint(&self, fingerprint: &str) {
        let fp_path = self.get_fingerprint_path();
        let mut file = match OpenOptions::new().create(true).write(true).truncate(true).open(fp_path) {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("unable to open fingerprint file for writing: {e}");
                return;
            }
        };
        if let Err(e) = write!(file, "{fingerprint}") {
            tracing::error!("unable to write fingerprint file: {e}");
        }
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
    fn get_cache_fingerprint(&mut self) -> Option<String> {
        let fp_path = self.get_fingerprint_path();
        match std::fs::read_to_string(fp_path) {
            Ok(fingerprint) => Some(fingerprint.trim().into()),
            Err(_) => None,
        }
    }
    fn load_from_cache(&mut self) -> Result<rc::ad::AData, Box<dyn std::error::Error>> {
        let full_path = self.get_cache_path();
        let file = OpenOptions::new()
            .read(true)
            .open(full_path)
            .map_err(|e| JsonZfileAdcError::ReadFailed(e.to_string()))?;
        let reader = zstd::stream::Decoder::new(file).map_err(|e| JsonZfileAdcError::ReadFailed(e.to_string()))?;
        let c_data = CData::try_deserialize(reader)?;
        Ok(c_data.into_adapted())
    }
    #[tracing::instrument(name = "adc-json-zfile-update", level = "trace", skip_all)]
    fn write_cache(&mut self, a_data: &rc::ad::AData, fingerprint: &str) {
        if let Some(err_str) = self.create_cache_folder() {
            tracing::error!("unable to create cache folder: {err_str}");
            return;
        }
        self.write_data(CData::from_adapted(a_data));
        self.write_fingerprint(fingerprint);
    }
    fn get_cacher_version(&self) -> String {
        VERSION.to_string()
    }
}
