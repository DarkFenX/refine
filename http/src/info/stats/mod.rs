pub(crate) use details::{
    HStatCapSim, HStatDmgEntry, HStatEhp, HStatErps, HStatHp, HStatInJam, HStatMining, HStatOutReps, HStatResists,
    HStatResource, HStatRps, HStatSensors, HStatSlot,
};
pub(crate) use fit::HFitStats;
pub(crate) use fleet::HFleetStats;
pub(crate) use item::HItemStats;

mod details;
mod fit;
mod fleet;
mod item;
