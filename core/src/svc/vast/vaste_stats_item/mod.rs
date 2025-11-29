pub(crate) use cap::StatCapSimStaggerInt;
pub use cap::{StatCapConsumerOptions, StatCapRegenOptions, StatCapSim, StatCapSimStagger, StatCapSrcKinds};
pub use tank::{StatLayerEhp, StatLayerErps, StatLayerErpsRegen, StatLayerHp, StatLayerRps, StatLayerRpsRegen};

mod cap;
mod checks;
mod tank;
mod vaste_dmg;
mod vaste_jam;
mod vaste_mining;
mod vaste_misc;
mod vaste_mobility;
mod vaste_neut;
mod vaste_rr;
mod vaste_sensors;
