pub(crate) use details::{
    HStatCapSim, HStatDmg, HStatLayerEhp, HStatLayerErps, HStatLayerErpsRegen, HStatLayerRps, HStatLayerRpsRegen,
    HStatMining, HStatTank, HStatTankRegen,
};
pub(crate) use fit::HFitStats;
pub(crate) use fleet::HFleetStats;
pub(crate) use item::HItemStats;

mod details;
mod fit;
mod fleet;
mod item;
