use crate::sol::AttrVal;

#[derive(Debug)]
pub struct BreacherRelDmgError {
    pub value: AttrVal,
}
impl std::error::Error for BreacherRelDmgError {}
impl std::fmt::Display for BreacherRelDmgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "breacher relative damage {} is out of allowed range [0, 1]",
            self.value
        )
    }
}
