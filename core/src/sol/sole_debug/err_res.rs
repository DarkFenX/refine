#[derive(Debug)]
pub(in crate::sol) struct SolDebugError {}
impl SolDebugError {
    pub(in crate::sol) fn new() -> Self {
        Self {}
    }
}
impl std::error::Error for SolDebugError {}
impl std::fmt::Display for SolDebugError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "debug error")
    }
}

pub(in crate::sol) type SolDebugResult = Result<(), SolDebugError>;
