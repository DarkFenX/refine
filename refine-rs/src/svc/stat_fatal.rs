use crate::stats::err::{StatFitShipAppliedError, StatItemAppliedError};

pub(crate) trait StatErrorFatality {
    fn is_fatal(&self) -> bool;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit errors
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatErrorFatality for StatFitShipAppliedError<!> {
    fn is_fatal(&self) -> bool {
        match self {
            Self::NoShip(_) => true,
            Self::ItemNotLoaded(_) => true,
            Self::UnsupportedStat(_) => true,
            Self::StatSpecific(_) => false,
            Self::ProjecteeNotFound(_) => false,
            Self::ProjecteeCantTakeProjs(_) => false,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item errors
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatErrorFatality for StatItemAppliedError<!> {
    fn is_fatal(&self) -> bool {
        match self {
            Self::ItemNotLoaded(_) => true,
            Self::UnsupportedStat(_) => true,
            Self::StatSpecific(_) => false,
            Self::ProjecteeNotFound(_) => false,
            Self::ProjecteeCantTakeProjs(_) => false,
        }
    }
}
