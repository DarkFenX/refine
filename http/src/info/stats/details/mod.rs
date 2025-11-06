pub(crate) use cap::HStatCapSim;
pub(crate) use dmg::HStatDmg;
pub(crate) use resource::HStatRes;
pub(crate) use sensor::HStatSensor;
pub(crate) use slot::HStatSlot;
pub(crate) use tank::{HStatLayerEhp, HStatLayerErps, HStatLayerHp, HStatLayerResist, HStatLayerRps, HStatTank};

mod cap;
mod dmg;
mod resource;
mod sensor;
mod slot;
mod tank;
