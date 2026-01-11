//! Stats-related exports
// TODO: remove dmg kinds once everything is moved to specific stat structs
pub use crate::{
    misc::DmgKinds,
    svc::vast::{
        StatCapRegenOptions, StatCapSim, StatCapSimStagger, StatCapSrcKinds, StatDmg, StatDmgApplied, StatDmgBreacher,
        StatDmgItemKinds, StatEhp, StatEhpLayer, StatErps, StatErpsLayer, StatErpsLayerRegen, StatHp, StatHpLayer,
        StatJamApplied, StatMining, StatMiningItemKinds, StatNeutItemKinds, StatOutRepItemKinds, StatOutReps,
        StatResists, StatResistsLayer, StatResource, StatRps, StatRpsLayer, StatRpsLayerRegen, StatSensors,
        StatSensorsKind, StatSlot, StatTimeOptions, StatTimeOptionsBurst, StatTimeOptionsSim,
    },
};
