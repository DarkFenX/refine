use crate::sol::AttrVal;

#[derive(Debug)]
pub struct TotalDmgError {
    pub value: AttrVal,
}
impl std::error::Error for TotalDmgError {}
impl std::fmt::Display for TotalDmgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "total damage {} is negative or zero", self.value)
    }
}
