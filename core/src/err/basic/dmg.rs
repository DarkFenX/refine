use crate::misc::Value;

#[derive(thiserror::Error, Debug)]
pub enum DmgError {
    #[error("EM damage {0} is negative")]
    Em(Value),
    #[error("thermal damage {0} is negative")]
    Thermal(Value),
    #[error("kinetic damage {0} is negative")]
    Kinetic(Value),
    #[error("explosive damage {0} is negative")]
    Explosive(Value),
}
