#[derive(thiserror::Error, Debug)]
pub(super) enum JsonZfileAdcDataReadError {
    #[error("reading failed")]
    Read(#[source] std::io::Error),
    #[error("parsing failed")]
    Parse(#[source] serde_json::Error),
}
impl From<serde_json::Error> for JsonZfileAdcDataReadError {
    fn from(error: serde_json::Error) -> Self {
        match error.is_io() {
            true => Self::Read(error.into()),
            false => Self::Parse(error),
        }
    }
}
impl From<JsonZfileAdcDataReadError> for rc::ad::err::AdaptedDataCacherError {
    fn from(error: JsonZfileAdcDataReadError) -> Self {
        Self::new(error)
    }
}

#[derive(thiserror::Error, Debug)]
pub(super) enum JsonZfileAdcFpReadError {
    #[error("reading failed")]
    Read(#[source] std::io::Error),
}
impl From<JsonZfileAdcFpReadError> for rc::ad::err::AdaptedDataCacherError {
    fn from(error: JsonZfileAdcFpReadError) -> Self {
        Self::new(error)
    }
}

#[derive(thiserror::Error, Debug)]
pub(super) enum JsonZfileAdcWriteError {
    #[error("unable to create directory")]
    CreateDir(#[source] std::io::Error),
    #[error("unable to write data")]
    DataWrite(#[source] std::io::Error),
    #[error("unable to serialize data")]
    DataSerialize(#[source] serde_json::Error),
    #[error("unable to write fingerprint")]
    FpWrite(#[source] std::io::Error),
}
impl From<std::io::Error> for JsonZfileAdcWriteError {
    fn from(error: std::io::Error) -> Self {
        Self::DataWrite(error)
    }
}
impl From<serde_json::Error> for JsonZfileAdcWriteError {
    fn from(error: serde_json::Error) -> Self {
        match error.is_io() {
            true => Self::DataWrite(error.into()),
            false => Self::DataSerialize(error),
        }
    }
}
impl From<JsonZfileAdcWriteError> for rc::ad::err::AdaptedDataCacherError {
    fn from(error: JsonZfileAdcWriteError) -> Self {
        Self::new(error)
    }
}
