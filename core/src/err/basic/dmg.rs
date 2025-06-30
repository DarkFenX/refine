use crate::def::AttrVal;

#[derive(thiserror::Error, Debug)]
pub enum DmgError {
    #[error("EM damage {0} is negative")]
    Em(AttrVal),
    #[error("thermal damage {0} is negative")]
    Thermal(AttrVal),
    #[error("kinetic damage {0} is negative")]
    Kinetic(AttrVal),
    #[error("explosive damage {0} is negative")]
    Explosive(AttrVal),
}
