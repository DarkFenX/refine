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
impl From<JsonZfsAdcDataReadError> for rc::ad::err::AdaptedDataCacherError {
    fn from(error: JsonZfsAdcDataReadError) -> Self {
        Self::new(error)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum JsonZfsAdcFpReadError {
    #[error("reading failed")]
    Read(#[source] std::io::Error),
}
impl From<JsonZfsAdcFpReadError> for rc::ad::err::AdaptedDataCacherError {
    fn from(error: JsonZfsAdcFpReadError) -> Self {
        Self::new(error)
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
    #[error("unable to write fingerprint")]
    FpWrite(#[source] std::io::Error),
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
impl From<JsonZfsAdcWriteError> for rc::ad::err::AdaptedDataCacherError {
    fn from(error: JsonZfsAdcWriteError) -> Self {
        Self::new(error)
    }
}
