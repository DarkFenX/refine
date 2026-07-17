pub use rc::{
    CtlAffectors as StatAffectors,
    stats::{StatMining, StatMiningEntry, StatOutReps, StatResource},
};

pub use crate::{
    cmd::{GetFitStatsCmd, GetFleetStatsCmd, GetItemStatsCmd, StatOption, StatOptionExt, StatOptionMass},
    info::{FitStats, FleetStats, ItemStats},
};
