use crate::sol::AttrVal;

#[derive(Debug)]
pub struct KinDmgError {
    pub value: AttrVal,
}
impl std::error::Error for KinDmgError {}
impl std::fmt::Display for KinDmgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "kinetic damage {} is negative", self.value)
    }
}
