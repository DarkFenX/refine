use crate::sol::AttrVal;

#[derive(thiserror::Error, Debug)]
#[error("total damage {value} is negative or zero")]
pub struct TotalDmgError {
    pub value: AttrVal,
}
