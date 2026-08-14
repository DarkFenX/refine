use std::{
    fs::{OpenOptions, create_dir_all},
    io::{self, BufReader, BufWriter, Write},
    path::PathBuf,
};

use super::error::{PostcardZfsAdcDataReadError, PostcardZfsAdcFpReadError, PostcardZfsAdcWriteError};
use crate::VERSION;

/// Postcard adapted data cacher implementation.
///
/// This cacher implements persistent cache store in the form of zstd-compressed postcard format.
pub struct PostcardZfsAdc {
    dir: PathBuf,
    name: String,
}
impl PostcardZfsAdc {
    /// Constructs new cacher using path to cache directory and cache file name (without extension).
    pub fn new(dir: impl Into<PathBuf>, name: impl Into<String>) -> Self {
        Self {
            dir: dir.into(),
            name: name.into(),
        }
    }
}
impl std::fmt::Debug for PostcardZfsAdc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PostcardZfsAdc(\"{}\")",
            self.get_cache_path().to_str().unwrap_or("<error>")
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cacher trait implementation
////////////////////////////////////////////////////////////////////////////////////////////////////
impl rc::ad::AdaptedDataCacherCore for PostcardZfsAdc {
    fn get_cache_fingerprint(&self) -> Result<rc::ad::AFingerprint, rc::ad::err::AdaptedDataCacherError> {
        let fingerprint =
            std::fs::read_to_string(self.get_fingerprint_path()).map_err(PostcardZfsAdcFpReadError::Read)?;
        Ok(rc::ad::AFingerprint::from_string(fingerprint.trim().into()))
    }
    fn load_from_cache(&self) -> Result<rc::ad::AData, rc::ad::err::AdaptedDataCacherError> {
        let full_path = self.get_cache_path();
        let file = OpenOptions::new()
            .read(true)
            .open(full_path)
            .map_err(PostcardZfsAdcDataReadError::Read)?;
        let reader = zstd::stream::Decoder::new(file).map_err(PostcardZfsAdcDataReadError::Read)?;
        // Scratch buffer needs to be bigger than the longest string in cache. As of 2026-07-30,
        // the only stored strings are ADG warnings, and all of them are capped, so 64k is more than
        // enough.
        let mut scratch = vec![0; 64 * 1024];
        let (a_data, _) = postcard::from_io((BufReader::new(reader), scratch.as_mut_slice()))
            .map_err(PostcardZfsAdcDataReadError::from)?;
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

impl PostcardZfsAdc {
    fn get_cache_path(&self) -> PathBuf {
        self.dir.join(format!("{}.postcard.zst", self.name))
    }
    fn get_fingerprint_path(&self) -> PathBuf {
        self.dir.join(format!("{}.postcard.zst.fp", self.name))
    }
    fn create_cache_dir(&self) -> Result<(), PostcardZfsAdcWriteError> {
        match create_dir_all(&self.dir) {
            Ok(()) => Ok(()),
            Err(e) => {
                match e.kind() {
                    // It's fine if it already exists for our purposes
                    io::ErrorKind::AlreadyExists => Ok(()),
                    _ => Err(PostcardZfsAdcWriteError::CreateDir(e)),
                }
            }
        }
    }
    fn write_data(&self, a_data: &rc::ad::AData) -> Result<(), PostcardZfsAdcWriteError> {
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
    fn write_fingerprint(&self, fingerprint: rc::ad::AFingerprint) -> Result<(), PostcardZfsAdcWriteError> {
        let fp_path = self.get_fingerprint_path();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(fp_path)
            .map_err(PostcardZfsAdcWriteError::FpWrite)?;
        write!(file, "{fingerprint}").map_err(PostcardZfsAdcWriteError::FpWrite)?;
        Ok(())
    }
}
