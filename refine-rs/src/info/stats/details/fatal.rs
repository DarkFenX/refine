use crate::stats::err::{
    AgilityStatError, FitAppliedStatError, FitCharacterStatError, FitShipAppliedStatError, FitShipStatError,
    FleetStatAppliedError, ItemAppliedStatError, ItemStatError,
};

pub(crate) trait StatError {
    fn is_fatal(&self) -> bool;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fleet errors
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatError for FleetStatAppliedError {
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
impl StatError for FitAppliedStatError {
    fn is_fatal(&self) -> bool {
        match self {
            Self::ProjecteeNotFound(_) => false,
            Self::ProjecteeCantTakeProjs(_) => false,
        }
    }
}

impl<SS> StatError for FitShipStatError<SS>
where
    SS: std::error::Error + StatError,
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

impl<SS> StatError for FitShipAppliedStatError<SS>
where
    SS: std::error::Error + StatError,
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

impl<SS> StatError for FitCharacterStatError<SS>
where
    SS: std::error::Error + StatError,
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
impl<SS> StatError for ItemStatError<SS>
where
    SS: std::error::Error + StatError,
{
    fn is_fatal(&self) -> bool {
        match self {
            Self::ItemNotLoaded(_) => true,
            Self::UnsupportedStat(_) => true,
            Self::StatSpecific(err) => err.is_fatal(),
        }
    }
}

impl<SS> StatError for ItemAppliedStatError<SS>
where
    SS: std::error::Error + StatError,
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
impl StatError for AgilityStatError {
    fn is_fatal(&self) -> bool {
        match self {
            Self::AgilityError(_) => true,
            Self::MassError(_) => true,
        }
    }
}
