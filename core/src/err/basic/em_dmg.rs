use crate::sol::AttrVal;

#[derive(Debug)]
pub struct EmDmgError {
    pub value: AttrVal,
}
impl std::error::Error for EmDmgError {}
impl std::fmt::Display for EmDmgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "EM damage {} is negative", self.value)
    }
}
