use crate::stats::err::{
    AgilityStatError, FitAppliedStatError, FitCharacterStatError, FitShipAppliedStatError, FitShipStatError,
    FleetAppliedStatError, ItemAppliedStatError, ItemStatError,
};

pub(crate) trait StatErrorFatality {
    fn is_fatal(&self) -> bool;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fleet errors
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatErrorFatality for FleetAppliedStatError {
    fn is_fatal(&self) -> bool {
        match self {
            Self::ProjecteeNotFound(_) => false,
            Self::ProjecteeCantTakeProjs(_) => false,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit errors
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatErrorFatality for FitAppliedStatError {
    fn is_fatal(&self) -> bool {
        match self {
            Self::ProjecteeNotFound(_) => false,
            Self::ProjecteeCantTakeProjs(_) => false,
        }
    }
}

impl<SS> StatErrorFatality for FitShipStatError<SS>
where
    SS: std::error::Error + StatErrorFatality,
{
    fn is_fatal(&self) -> bool {
        match self {
            Self::NoShip(_) => true,
            Self::ItemNotLoaded(_) => true,
            Self::UnsupportedStat(_) => true,
            Self::StatSpecific(stat_err) => stat_err.is_fatal(),
        }
    }
}

impl<SS> StatErrorFatality for FitShipAppliedStatError<SS>
where
    SS: std::error::Error + StatErrorFatality,
{
    fn is_fatal(&self) -> bool {
        match self {
            Self::NoShip(_) => true,
            Self::ItemNotLoaded(_) => true,
            Self::UnsupportedStat(_) => true,
            Self::StatSpecific(stat_err) => stat_err.is_fatal(),
            Self::ProjecteeNotFound(_) => false,
            Self::ProjecteeCantTakeProjs(_) => false,
        }
    }
}
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

impl<SS> StatErrorFatality for FitCharacterStatError<SS>
where
    SS: std::error::Error + StatErrorFatality,
{
    fn is_fatal(&self) -> bool {
        match self {
            Self::NoCharacter(_) => true,
            Self::ItemNotLoaded(_) => true,
            Self::UnsupportedStat(_) => true,
            Self::StatSpecific(stat_err) => stat_err.is_fatal(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item errors
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<SS> StatErrorFatality for ItemStatError<SS>
where
    SS: std::error::Error + StatErrorFatality,
{
    fn is_fatal(&self) -> bool {
        match self {
            Self::ItemNotLoaded(_) => true,
            Self::UnsupportedStat(_) => true,
            Self::StatSpecific(err) => err.is_fatal(),
        }
    }
}

impl<SS> StatErrorFatality for ItemAppliedStatError<SS>
where
    SS: std::error::Error + StatErrorFatality,
{
    fn is_fatal(&self) -> bool {
        match self {
            Self::ItemNotLoaded(_) => true,
            Self::UnsupportedStat(_) => true,
            Self::StatSpecific(err) => err.is_fatal(),
            Self::ProjecteeNotFound(_) => false,
            Self::ProjecteeCantTakeProjs(_) => false,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Stat-specific errors
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatErrorFatality for AgilityStatError {
    fn is_fatal(&self) -> bool {
        match self {
            Self::AgilityError(_) => true,
            Self::MassError(_) => true,
        }
    }
}
