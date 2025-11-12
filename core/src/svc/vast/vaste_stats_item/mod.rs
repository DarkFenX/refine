pub(crate) use cap::StatCapSimStaggerInt;
pub use cap::{StatCapConsumerOptions, StatCapRegenOptions, StatCapSim, StatCapSimStagger, StatCapSrcKinds};
pub use tank::{StatLayerEhp, StatLayerErps, StatLayerHp, StatLayerRps};

mod cap;
mod checks;
mod tank;
mod vaste_dmg;
mod vaste_misc;
mod vaste_neut;
mod vaste_physics;
mod vaste_rr;
mod vaste_sensors;
