use crate::def::AttrVal;

#[derive(thiserror::Error, Debug)]
pub enum BreacherDmgError {
    #[error("breacher absolute damage {0} is negative")]
    Absolute(AttrVal),
    #[error("breacher relative damage {0} is out of allowed range [0, 1]")]
    Relative(AttrVal),
}
