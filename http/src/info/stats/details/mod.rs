pub(crate) use cap::HStatCapSim;
pub(crate) use dmg::HStatDmg;
pub(crate) use mining::HStatMining;
pub(crate) use resource::HStatRes;
pub(crate) use sensor::HStatSensor;
pub(crate) use slot::HStatSlot;
pub(crate) use tank::{
    HStatLayerEhp, HStatLayerErps, HStatLayerErpsRegen, HStatLayerHp, HStatLayerResist, HStatLayerRps,
    HStatLayerRpsRegen, HStatTank, HStatTankRegen,
};

mod cap;
mod dmg;
mod mining;
mod resource;
mod sensor;
mod slot;
mod tank;
