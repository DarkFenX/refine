pub(crate) use cap::HStatCapSim;
pub(crate) use dmg::HStatDmg;
pub(crate) use jam::HStatJamApplied;
pub(crate) use mining::HStatMining;
pub(crate) use out_reps::HStatOutReps;
pub(crate) use resource::HStatResource;
pub(crate) use sensors::HStatSensors;
pub(crate) use slot::HStatSlot;
pub(crate) use tank::{HStatEhp, HStatErps, HStatHp, HStatResists, HStatRps};

mod cap;
mod dmg;
mod jam;
mod mining;
mod out_reps;
mod resource;
mod sensors;
mod slot;
mod tank;
