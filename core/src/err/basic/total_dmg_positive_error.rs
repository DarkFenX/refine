use crate::sol::AttrVal;

#[derive(Debug)]
pub struct TotalDmgPositiveError {
    pub value: AttrVal,
}
impl TotalDmgPositiveError {
    pub(crate) fn new(value: AttrVal) -> Self {
        Self { value }
    }
}
impl std::error::Error for TotalDmgPositiveError {}
impl std::fmt::Display for TotalDmgPositiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "total damage {} is negative or zero", self.value)
    }
}
