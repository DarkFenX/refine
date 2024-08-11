#[derive(Debug)]
pub(crate) enum HCoreError {
    FitNotFound(rc::err::basic::FitFoundError),
}
impl HCoreError {
    pub(crate) fn get_code(&self) -> String {
        "fake".to_string()
    }
}
impl From<rc::Error> for HCoreError {
    fn from(core_error: rc::Error) -> Self {
        Self { error: core_error }
    }
}
impl std::error::Error for HCoreError {}
impl std::fmt::Display for HCoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "core library error")
    }
}
