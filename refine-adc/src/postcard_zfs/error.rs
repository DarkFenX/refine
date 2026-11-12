use crate::zfs_shared::ZfsFpHeaderError;

#[derive(thiserror::Error, Debug)]
pub enum PostcardZfsAdcDataReadError {
    #[error("reading failed")]
    Read(#[source] std::io::Error),
    #[error("parsing failed")]
    Parse(#[source] postcard::Error),
}
impl From<PostcardZfsAdcDataReadError> for rc::ad::err::AdcLoadError {
    fn from(error: PostcardZfsAdcDataReadError) -> Self {
        Self::DataRead(Box::new(error))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PostcardZfsAdcFpReadError {
    #[error("reading failed")]
    Read(#[source] std::io::Error),
    #[error("fingerprint header is invalid")]
    Header,
}
impl From<ZfsFpHeaderError> for PostcardZfsAdcFpReadError {
    fn from(error: ZfsFpHeaderError) -> Self {
        match error {
            ZfsFpHeaderError::Read(error) => Self::Read(error),
            ZfsFpHeaderError::Invalid => Self::Header,
        }
    }
}
impl From<PostcardZfsAdcFpReadError> for rc::ad::err::AdcLoadError {
    fn from(error: PostcardZfsAdcFpReadError) -> Self {
        Self::FingerprintRead(Box::new(error))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PostcardZfsAdcWriteError {
    #[error("unable to create directory")]
    CreateDir(#[source] std::io::Error),
    #[error("unable to write data")]
    DataWrite(#[source] std::io::Error),
    #[error("unable to serialize data")]
    DataSerialize(#[source] postcard::Error),
}
impl From<std::io::Error> for PostcardZfsAdcWriteError {
    fn from(error: std::io::Error) -> Self {
        Self::DataWrite(error)
    }
}
impl From<PostcardZfsAdcWriteError> for rc::ad::err::AdcWriteError {
    fn from(error: PostcardZfsAdcWriteError) -> Self {
        Self(Box::new(error))
    }
}
