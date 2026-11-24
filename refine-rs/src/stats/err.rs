pub use crate::stats::{
    fit::{FitGetFitStatsError, FitStatsEnumError},
    fleet::FleetGetFleetStatsError,
    item::{FitItemGetItemStatsError, ItemGetItemStatsError},
    sol::SolStatsEnumError,
};

pub mod core {
    pub use rc::stats::err::*;
}
