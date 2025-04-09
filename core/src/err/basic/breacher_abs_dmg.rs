use crate::sol::AttrVal;

#[derive(thiserror::Error, Debug)]
#[error("breacher damage {value} is negative")]
pub struct BreacherAbsDmgError {
    pub value: AttrVal,
}
