#[derive(thiserror::Error, Debug)]
pub(super) enum PostcardZfileAdcDataReadError {
    #[error("reading failed")]
    Read(#[source] std::io::Error),
    #[error("parsing failed")]
    Parse(#[source] postcard::Error),
}
impl From<postcard::Error> for PostcardZfileAdcDataReadError {
    fn from(error: postcard::Error) -> Self {
        Self::Parse(error)
    }
}
impl From<PostcardZfileAdcDataReadError> for rc::ad::err::AdaptedDataCacherError {
    fn from(error: PostcardZfileAdcDataReadError) -> Self {
        Self::new(error)
    }
}

#[derive(thiserror::Error, Debug)]
pub(super) enum PostcardZfileAdcFpReadError {
    #[error("reading failed")]
    Read(#[source] std::io::Error),
}
impl From<PostcardZfileAdcFpReadError> for rc::ad::err::AdaptedDataCacherError {
    fn from(error: PostcardZfileAdcFpReadError) -> Self {
        Self::new(error)
    }
}

#[derive(thiserror::Error, Debug)]
pub(super) enum PostcardZfileAdcWriteError {
    #[error("unable to create directory")]
    CreateDir(#[source] std::io::Error),
    #[error("unable to write data")]
    DataWrite(#[source] std::io::Error),
    #[error("unable to serialize data")]
    DataSerialize(#[source] postcard::Error),
    #[error("unable to write fingerprint")]
    FpWrite(#[source] std::io::Error),
}
impl From<std::io::Error> for PostcardZfileAdcWriteError {
    fn from(error: std::io::Error) -> Self {
        Self::DataWrite(error)
    }
}
impl From<postcard::Error> for PostcardZfileAdcWriteError {
    fn from(error: postcard::Error) -> Self {
        Self::DataSerialize(error)
    }
}
impl From<PostcardZfileAdcWriteError> for rc::ad::err::AdaptedDataCacherError {
    fn from(error: PostcardZfileAdcWriteError) -> Self {
        Self::new(error)
    }
}
