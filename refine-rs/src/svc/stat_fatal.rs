use crate::stats::err::{FitShipAppliedStatError, ItemAppliedStatError};

pub(crate) trait StatErrorFatality {
    fn is_fatal(&self) -> bool;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit errors
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatErrorFatality for FitShipAppliedStatError<!> {
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
impl StatErrorFatality for ItemAppliedStatError<!> {
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
