#[derive(thiserror::Error, Default, Debug)]
#[error("debug error")]
pub(crate) struct DebugError;

pub(crate) type DebugResult = Result<(), DebugError>;
