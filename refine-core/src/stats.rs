//! Stats-related exports
pub use crate::{
    api::StatItemStateOptions,
    svc::vast::{
        StatCapBlcNosfs, StatCapBlcRegen, StatCapBlcSrcKinds, StatCapSim, StatCapSimStagger, StatCritOptions, StatDmg,
        StatDmgApplied, StatDmgEntry, StatDmgEntryApplied, StatDmgEntryBreacher, StatDmgItemKinds, StatEhp,
        StatEhpLayer, StatErps, StatErpsLayer, StatErpsLayerRegen, StatHp, StatHpLayer, StatInJam,
        StatItemChargeOptions, StatJump, StatJumpConduit, StatJumpPassenger, StatJumpPortal, StatJumpRange,
        StatJumpSelf, StatMining, StatMiningEntry, StatMiningItemKinds, StatNeutItemKinds, StatOutRepItemKinds,
        StatOutReps, StatResists, StatResistsLayer, StatResource, StatRps, StatRpsLayer, StatRpsLayerRegen,
        StatSensors, StatSensorsKind, StatSlot, StatTimeOptions, StatTimeOptionsBurst, StatTimeOptionsSim,
    },
};

pub mod err {
    pub use crate::{
        api::{
            StatFitAppliedError, StatFitCharacterError, StatFitShipAppliedError, StatFitShipError,
            StatFleetAppliedError, StatItemAppliedError, StatItemError,
        },
        svc::vast::{StatAgilityError, StatJumpError, StatMaxWarpRangeError, StatProbingSizeError, StatWarpSpeedError},
    };
}
