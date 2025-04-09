use crate::sol::AttrVal;

#[derive(thiserror::Error, Debug)]
#[error("EM damage {value} is negative")]
pub struct EmDmgError {
    pub value: AttrVal,
}
