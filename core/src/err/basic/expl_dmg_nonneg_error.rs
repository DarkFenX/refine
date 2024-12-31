use crate::defs::AttrVal;

#[derive(Debug)]
pub struct ExplDmgNonNegError {
    pub value: AttrVal,
}
impl ExplDmgNonNegError {
    pub(crate) fn new(value: AttrVal) -> Self {
        Self { value }
    }
}
impl std::error::Error for ExplDmgNonNegError {}
impl std::fmt::Display for ExplDmgNonNegError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "explosive damage {} is negative", self.value)
    }
}
