use crate::sol::AttrVal;

#[derive(Debug)]
pub struct ThermDmgNonNegError {
    pub value: AttrVal,
}
impl ThermDmgNonNegError {
    pub(crate) fn new(value: AttrVal) -> Self {
        Self { value }
    }
}
impl std::error::Error for ThermDmgNonNegError {}
impl std::fmt::Display for ThermDmgNonNegError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "thermal damage {} is negative", self.value)
    }
}
