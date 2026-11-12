use std::{
    fs::{File, create_dir_all},
    io::{self, BufReader, Read, Write},
    path::PathBuf,
};

use super::error::{PostcardZfsAdcDataReadError, PostcardZfsAdcFpReadError, PostcardZfsAdcWriteError};
use crate::{VERSION, zfs_shared};

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
    fn load_from_cache(&self, fingerprint: &rc::ad::AFingerprint) -> Result<rc::ad::AData, rc::ad::err::AdcLoadError> {
        let file = File::open(self.get_cache_path()).map_err(PostcardZfsAdcFpReadError::Read)?;
        let mut reader = BufReader::new(file);
        let cached_fingerprint = zfs_shared::read_fp(&mut reader).map_err(PostcardZfsAdcFpReadError::from)?;
        if cached_fingerprint != *fingerprint {
            return Err(rc::ad::err::AdcLoadError::FingerprintMismatch(cached_fingerprint));
        }
        let mut decoder = zstd::stream::Decoder::with_buffer(reader).map_err(PostcardZfsAdcDataReadError::Read)?;
        // Data is decompressed before parsing, so that zstd verifies checksum at the end of frame
        // before corrupted data reaches the parser
        let mut bytes = Vec::new();
        decoder
            .read_to_end(&mut bytes)
            .map_err(PostcardZfsAdcDataReadError::Read)?;
        let a_data = postcard::from_bytes(&bytes).map_err(PostcardZfsAdcDataReadError::Parse)?;
        Ok(a_data)
    }
    fn write_cache(
        &self,
        a_data: &rc::ad::AData,
        fingerprint: rc::ad::AFingerprint,
    ) -> Result<(), rc::ad::err::AdcWriteError> {
        self.create_cache_dir()?;
        zfs_shared::write(&self.get_cache_path(), &fingerprint, |writer| {
            // Data is serialized before writing, since it is compact, this way it's easier, plus
            // postcard does not keep I/O errors of writers, which results in even more code if
            // those errors need to be kept
            let bytes = postcard::to_stdvec(a_data).map_err(PostcardZfsAdcWriteError::DataSerialize)?;
            writer.write_all(&bytes).map_err(PostcardZfsAdcWriteError::DataWrite)
        })?;
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
}
