use crate::sol::AttrVal;

#[derive(thiserror::Error, Debug)]
#[error("explosive damage {value} is negative")]
pub struct ExplDmgError {
    pub value: AttrVal,
}
