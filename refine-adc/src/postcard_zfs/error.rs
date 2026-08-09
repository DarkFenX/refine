#[derive(thiserror::Error, Debug)]
pub enum PostcardZfsAdcDataReadError {
    #[error("reading failed")]
    Read(#[source] std::io::Error),
    #[error("parsing failed")]
    Parse(#[source] postcard::Error),
}
impl From<postcard::Error> for PostcardZfsAdcDataReadError {
    fn from(error: postcard::Error) -> Self {
        Self::Parse(error)
    }
}
impl From<PostcardZfsAdcDataReadError> for rc::ad::err::AdaptedDataCacherError {
    fn from(error: PostcardZfsAdcDataReadError) -> Self {
        Self::new(error)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PostcardZfsAdcFpReadError {
    #[error("reading failed")]
    Read(#[source] std::io::Error),
}
impl From<PostcardZfsAdcFpReadError> for rc::ad::err::AdaptedDataCacherError {
    fn from(error: PostcardZfsAdcFpReadError) -> Self {
        Self::new(error)
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
    #[error("unable to write fingerprint")]
    FpWrite(#[source] std::io::Error),
}
impl From<std::io::Error> for PostcardZfsAdcWriteError {
    fn from(error: std::io::Error) -> Self {
        Self::DataWrite(error)
    }
}
impl From<postcard::Error> for PostcardZfsAdcWriteError {
    fn from(error: postcard::Error) -> Self {
        Self::DataSerialize(error)
    }
}
impl From<PostcardZfsAdcWriteError> for rc::ad::err::AdaptedDataCacherError {
    fn from(error: PostcardZfsAdcWriteError) -> Self {
        Self::new(error)
    }
}
