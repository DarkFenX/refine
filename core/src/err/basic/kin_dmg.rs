use crate::sol::AttrVal;

#[derive(thiserror::Error, Debug)]
#[error("kinetic damage {value} is negative")]
pub struct KinDmgError {
    pub value: AttrVal,
}
