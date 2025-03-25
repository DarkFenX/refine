use crate::sol::AttrVal;

#[derive(Debug)]
pub struct KinDmgNonNegError {
    pub value: AttrVal,
}
impl KinDmgNonNegError {
    pub(crate) fn new(value: AttrVal) -> Self {
        Self { value }
    }
}
impl std::error::Error for KinDmgNonNegError {}
impl std::fmt::Display for KinDmgNonNegError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "kinetic damage {} is negative", self.value)
    }
}
