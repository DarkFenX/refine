use crate::sol::AttrVal;

#[derive(Debug)]
pub struct ThermDmgError {
    pub value: AttrVal,
}
impl std::error::Error for ThermDmgError {}
impl std::fmt::Display for ThermDmgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "thermal damage {} is negative", self.value)
    }
}
