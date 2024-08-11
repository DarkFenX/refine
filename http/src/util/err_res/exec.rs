use crate::util::HCoreError;

#[derive(Debug)]
pub(crate) enum HExecErrorKind {
    CoreError(HCoreError),
}

#[derive(Debug)]
pub(crate) struct HExecError {
    pub(crate) kind: HExecErrorKind,
}
impl HExecError {
    pub(crate) fn new(kind: HExecErrorKind) -> Self {
        Self { kind }
    }
    pub(crate) fn get_code(&self) -> String {
        match &self.kind {
            HExecErrorKind::CoreError(e) => e.get_code(),
        }
    }
}
impl std::error::Error for HExecError {}
impl std::fmt::Display for HExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            HExecErrorKind::CoreError(e) => write!(f, "{e}"),
        }
    }
}

pub(crate) type HExecResult<T> = Result<T, HExecError>;
