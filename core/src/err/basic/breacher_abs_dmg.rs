use crate::sol::AttrVal;

#[derive(Debug)]
pub struct BreacherAbsDmgError {
    pub value: AttrVal,
}
impl std::error::Error for BreacherAbsDmgError {}
impl std::fmt::Display for BreacherAbsDmgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "breacher damage {} is negative", self.value)
    }
}
