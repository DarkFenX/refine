use crate::sol::AttrVal;

#[derive(thiserror::Error, Debug)]
#[error("breacher relative damage {value} is out of allowed range [0, 1]")]
pub struct BreacherRelDmgError {
    pub value: AttrVal,
}
