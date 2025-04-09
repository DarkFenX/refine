use crate::sol::AttrVal;

#[derive(thiserror::Error, Debug)]
#[error("thermal damage {value} is negative")]
pub struct ThermDmgError {
    pub value: AttrVal,
}
