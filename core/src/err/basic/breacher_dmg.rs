use crate::misc::Value;

#[derive(thiserror::Error, Debug)]
pub enum BreacherDmgError {
    #[error("breacher absolute damage {0} is negative")]
    Absolute(Value),
    #[error("breacher relative damage {0} is out of allowed range [0, 1]")]
    Relative(Value),
}
