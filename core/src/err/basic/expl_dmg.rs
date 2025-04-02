use crate::sol::AttrVal;

#[derive(Debug)]
pub struct ExplDmgError {
    pub value: AttrVal,
}
impl std::error::Error for ExplDmgError {}
impl std::fmt::Display for ExplDmgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "explosive damage {} is negative", self.value)
    }
}
