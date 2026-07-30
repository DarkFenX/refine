use std::{
    fmt,
    fs::{OpenOptions, create_dir_all},
    io::{self, BufReader, BufWriter, Write},
    path::PathBuf,
};

use super::error::{PostcardZfileAdcDataReadError, PostcardZfileAdcFpReadError, PostcardZfileAdcWriteError};
use crate::VERSION;

/// Postcard adapted data cacher implementation.
///
/// This cacher implements persistent cache store in the form of zstd-compressed postcard format.
pub struct PostcardZfileAdc {
    dir: PathBuf,
    name: String,
}
impl PostcardZfileAdc {
    /// Constructs new cacher using path to cache directory and cache file name (without extension).
    pub fn new(dir: PathBuf, name: String) -> Self {
        Self { dir, name }
    }
    fn get_cache_path(&self) -> PathBuf {
        self.dir.join(format!("{}.postcard.zst", self.name))
    }
    fn get_fingerprint_path(&self) -> PathBuf {
        self.dir.join(format!("{}.postcard.zst.fp", self.name))
    }
    fn create_cache_dir(&self) -> Result<(), PostcardZfileAdcWriteError> {
        match create_dir_all(&self.dir) {
            Ok(()) => Ok(()),
            Err(e) => {
                match e.kind() {
                    // It's fine if it already exists for our purposes
                    io::ErrorKind::AlreadyExists => Ok(()),
                    _ => Err(PostcardZfileAdcWriteError::CreateDirFailed(e.to_string())),
                }
            }
        }
    }
    fn write_data(&self, a_data: &rc::ad::AData) -> Result<(), PostcardZfileAdcWriteError> {
        let cache_path = self.get_cache_path();
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(cache_path)?;
        // zstd has internal input buffer, but it uses FFI which makes it moderately expensive;
        // postcard writes very few bytes at a time, so native write buffer actually helps
        let writer = BufWriter::new(zstd::stream::Encoder::new(file, 7)?.auto_finish());
        postcard::to_io(a_data, writer)?;
        Ok(())
    }
    fn write_fingerprint(&self, fingerprint: rc::ad::AFingerprint) -> Result<(), PostcardZfileAdcWriteError> {
        let fp_path = self.get_fingerprint_path();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(fp_path)
            .map_err(|e| PostcardZfileAdcWriteError::FpWriteFailed(e.to_string()))?;
        write!(file, "{fingerprint}").map_err(|e| PostcardZfileAdcWriteError::FpWriteFailed(e.to_string()))?;
        Ok(())
    }
}
impl fmt::Debug for PostcardZfileAdc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PostcardZfileAdc(\"{}\")",
            self.get_cache_path().to_str().unwrap_or("<error>")
        )
    }
}
impl rc::ad::AdaptedDataCacher for PostcardZfileAdc {
    fn get_cache_fingerprint(&mut self) -> Result<rc::ad::AFingerprint, Box<dyn std::error::Error>> {
        let fingerprint = std::fs::read_to_string(self.get_fingerprint_path())
            .map_err(|e| PostcardZfileAdcFpReadError::ReadFailed(e.to_string()))?;
        Ok(rc::ad::AFingerprint::from_string(fingerprint.trim().into()))
    }
    fn load_from_cache(&mut self) -> Result<rc::ad::AData, Box<dyn std::error::Error>> {
        let full_path = self.get_cache_path();
        let file = OpenOptions::new()
            .read(true)
            .open(full_path)
            .map_err(|e| PostcardZfileAdcDataReadError::ReadFailed(e.to_string()))?;
        let reader =
            zstd::stream::Decoder::new(file).map_err(|e| PostcardZfileAdcDataReadError::ReadFailed(e.to_string()))?;
        // Scratch buffer needs to be bigger than longest string in cache
        let mut scratch = vec![0; 64 * 1024];
        let (a_data, _) = postcard::from_io((BufReader::new(reader), scratch.as_mut_slice()))?;
        Ok(a_data)
    }
    fn write_cache(
        &mut self,
        a_data: &rc::ad::AData,
        fingerprint: rc::ad::AFingerprint,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.create_cache_dir()?;
        self.write_data(a_data)?;
        self.write_fingerprint(fingerprint)?;
        Ok(())
    }
    fn get_cacher_version(&self) -> String {
        VERSION.to_string()
    }
}
