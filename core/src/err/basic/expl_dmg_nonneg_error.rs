use crate::sol::AttrVal;

#[derive(Debug)]
pub struct ExplDmgNonNegError {
    pub value: AttrVal,
}
impl std::error::Error for ExplDmgNonNegError {}
impl std::fmt::Display for ExplDmgNonNegError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "explosive damage {} is negative", self.value)
    }
}
