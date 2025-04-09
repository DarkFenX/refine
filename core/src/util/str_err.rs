#[derive(thiserror::Error, Debug)]
#[error("{msg}")]
pub(crate) struct StrMsgError {
    pub(crate) msg: String,
}
