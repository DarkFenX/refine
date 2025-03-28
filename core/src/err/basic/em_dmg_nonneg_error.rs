use crate::sol::AttrVal;

#[derive(Debug)]
pub struct EmDmgNonNegError {
    pub value: AttrVal,
}
impl std::error::Error for EmDmgNonNegError {}
impl std::fmt::Display for EmDmgNonNegError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "EM damage {} is negative", self.value)
    }
}
