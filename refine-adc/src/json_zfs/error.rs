use crate::zfs_shared::ZfsFpHeaderError;

#[derive(thiserror::Error, Debug)]
pub enum JsonZfsAdcDataReadError {
    #[error("reading failed")]
    Read(#[source] std::io::Error),
    #[error("parsing failed")]
    Parse(#[source] serde_json::Error),
}
impl From<serde_json::Error> for JsonZfsAdcDataReadError {
    fn from(error: serde_json::Error) -> Self {
        match error.is_io() {
            true => Self::Read(error.into()),
            false => Self::Parse(error),
        }
    }
}
impl From<JsonZfsAdcDataReadError> for rc::ad::err::AdcLoadError {
    fn from(error: JsonZfsAdcDataReadError) -> Self {
        Self::DataRead(Box::new(error))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum JsonZfsAdcFpReadError {
    #[error("reading failed")]
    Read(#[source] std::io::Error),
    #[error("fingerprint header is invalid")]
    Header,
}
impl From<ZfsFpHeaderError> for JsonZfsAdcFpReadError {
    fn from(error: ZfsFpHeaderError) -> Self {
        match error {
            ZfsFpHeaderError::Read(error) => Self::Read(error),
            ZfsFpHeaderError::Invalid => Self::Header,
        }
    }
}
impl From<JsonZfsAdcFpReadError> for rc::ad::err::AdcLoadError {
    fn from(error: JsonZfsAdcFpReadError) -> Self {
        Self::FingerprintRead(Box::new(error))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum JsonZfsAdcWriteError {
    #[error("unable to create directory")]
    CreateDir(#[source] std::io::Error),
    #[error("unable to write data")]
    DataWrite(#[source] std::io::Error),
    #[error("unable to serialize data")]
    DataSerialize(#[source] serde_json::Error),
}
impl From<std::io::Error> for JsonZfsAdcWriteError {
    fn from(error: std::io::Error) -> Self {
        Self::DataWrite(error)
    }
}
impl From<serde_json::Error> for JsonZfsAdcWriteError {
    fn from(error: serde_json::Error) -> Self {
        match error.is_io() {
            true => Self::DataWrite(error.into()),
            false => Self::DataSerialize(error),
        }
    }
}
impl From<JsonZfsAdcWriteError> for rc::ad::err::AdcWriteError {
    fn from(error: JsonZfsAdcWriteError) -> Self {
        Self(Box::new(error))
    }
}
