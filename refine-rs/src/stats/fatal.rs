use crate::stats::err::{StatFitShipAppliedError, StatItemAppliedError};

pub(in crate::stats) trait StatErrorFatality {
    fn is_fatal(&self) -> bool;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit errors
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatErrorFatality for StatFitShipAppliedError<!> {
    fn is_fatal(&self) -> bool {
        match self {
            Self::NoShip(..) => true,
            Self::ItemNotLoaded(..) => true,
            Self::UnsupportedStat(..) => true,
            Self::StatSpecific(..) => false,
            Self::ProjecteeNotFound(..) => false,
            Self::ProjecteeCantTakeProjs(..) => false,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item errors
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatErrorFatality for StatItemAppliedError<!> {
    fn is_fatal(&self) -> bool {
        match self {
            Self::ItemNotLoaded(..) => true,
            Self::UnsupportedStat(..) => true,
            Self::StatSpecific(..) => false,
            Self::ProjecteeNotFound(..) => false,
            Self::ProjecteeCantTakeProjs(..) => false,
        }
    }
}
