pub(crate) use details::{
    HStatCapSim, HStatDmg, HStatEhp, HStatErps, HStatHp, HStatMining, HStatOutReps, HStatResists, HStatRps,
};
pub(crate) use fit::HFitStats;
pub(crate) use fleet::HFleetStats;
pub(crate) use item::HItemStats;

mod details;
mod fit;
mod fleet;
mod item;
