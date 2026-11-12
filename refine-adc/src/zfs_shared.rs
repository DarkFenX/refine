//! Cache file layout:
//! - zstd skippable frame: fingerprint
//! - zstd frame: content with checksum

use std::{
    ffi::OsString,
    fs::{File, OpenOptions},
    io::{self, BufWriter, IntoInnerError, Read, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

// Limits allocation when length in a corrupted header is read
const FP_LEN_MAX: u32 = 64 * 1024;
const ZSTD_LEVEL: i32 = 7;

pub(crate) type ZfsWriter = BufWriter<zstd::stream::Encoder<'static, File>>;

pub(crate) enum ZfsFpHeaderError {
    Read(io::Error),
    Invalid,
}

pub(crate) fn read_fp(reader: &mut impl Read) -> Result<rc::ad::AFingerprint, ZfsFpHeaderError> {
    let mut frame_header = [0; 8];
    reader.read_exact(&mut frame_header).map_err(ZfsFpHeaderError::Read)?;
    let [m0, m1, m2, m3, l0, l1, l2, l3] = frame_header;
    if u32::from_le_bytes([m0, m1, m2, m3]) != zstd::zstd_safe::MAGIC_SKIPPABLE_START {
        return Err(ZfsFpHeaderError::Invalid);
    }
    let fp_len = u32::from_le_bytes([l0, l1, l2, l3]);
    if fp_len > FP_LEN_MAX {
        return Err(ZfsFpHeaderError::Invalid);
    }
    let mut fp = vec![0; fp_len as usize];
    reader.read_exact(&mut fp).map_err(ZfsFpHeaderError::Read)?;
    String::from_utf8(fp)
        .map(rc::ad::AFingerprint::from_string)
        .map_err(|_| ZfsFpHeaderError::Invalid)
}

/// Writes cache file to a temporary path, and then renames it to the cache path.
pub(crate) fn write<E>(
    path: &Path,
    fingerprint: &rc::ad::AFingerprint,
    write_data: impl FnOnce(&mut ZfsWriter) -> Result<(), E>,
) -> Result<(), E>
where
    E: From<io::Error>,
{
    let tmp_path = get_tmp_path(path);
    let result =
        write_tmp(&tmp_path, fingerprint, write_data).and_then(|()| std::fs::rename(&tmp_path, path).map_err(E::from));
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp_path);
    }
    result
}

fn write_tmp<E>(
    tmp_path: &Path,
    fingerprint: &rc::ad::AFingerprint,
    write_data: impl FnOnce(&mut ZfsWriter) -> Result<(), E>,
) -> Result<(), E>
where
    E: From<io::Error>,
{
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(tmp_path)?;
    write_fp(&mut file, fingerprint)?;
    let mut encoder = zstd::stream::Encoder::new(file, ZSTD_LEVEL)?;
    encoder.include_checksum(true)?;
    // zstd has internal input buffer, but it uses FFI which makes it moderately expensive;
    // serializers write very few bytes at a time, so native write buffer actually helps
    let mut writer = BufWriter::new(encoder);
    write_data(&mut writer)?;
    // Flushing and finishing on drop would discard their errors
    let file = writer.into_inner().map_err(IntoInnerError::into_error)?.finish()?;
    // Some write errors are reported only on sync, and data has to be on disk before rename
    file.sync_all()?;
    Ok(())
}

fn write_fp(file: &mut File, fingerprint: &rc::ad::AFingerprint) -> io::Result<()> {
    let fp = fingerprint.get_str().as_bytes();
    let fp_len = u32::try_from(fp.len())
        .ok()
        .filter(|fp_len| *fp_len <= FP_LEN_MAX)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "fingerprint is too long"))?;
    let mut fp_frame = Vec::with_capacity(8 + fp.len());
    fp_frame.extend(zstd::zstd_safe::MAGIC_SKIPPABLE_START.to_le_bytes());
    fp_frame.extend(fp_len.to_le_bytes());
    fp_frame.extend(fp);
    file.write_all(&fp_frame)
}

fn get_tmp_path(path: &Path) -> PathBuf {
    // Process ID and sequence number keep concurrent writers to the same path apart
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let mut tmp_path = OsString::from(path);
    tmp_path.push(format!(
        ".{}-{}.tmp",
        std::process::id(),
        SEQ.fetch_add(1, Ordering::Relaxed)
    ));
    PathBuf::from(tmp_path)
}
