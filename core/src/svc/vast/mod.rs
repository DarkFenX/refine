pub use shared::{
    StatDmg, StatDmgApplied, StatDmgBreacher, StatDmgItemKinds, StatJamApplied, StatMining, StatMiningItemKinds,
    StatNeutItemKinds, StatOutRepItemKinds, StatSensors, StatSensorsKind, StatTank, StatTankRegen,
};
pub use val_options::{ValOption, ValOptions, ValOptionsSol};
pub(crate) use val_options::{ValOptionsInt, ValOptionsSolInt};
pub use val_result::{ValResultFit, ValResultSol};
pub(in crate::svc) use vast::Vast;
use vast::VastFitData;
pub(crate) use vaste_stats::StatCapSimStaggerInt;
pub use vaste_stats::{
    StatCapRegenOptions, StatCapSim, StatCapSimStagger, StatCapSrcKinds, StatLayerEhp, StatLayerErps,
    StatLayerErpsRegen, StatLayerHp, StatLayerRps, StatLayerRpsRegen, StatRes, StatSlot, StatTimeOptions,
    StatTimeOptionsBurst, StatTimeOptionsSim,
};
pub use vaste_vals::{
    ValActivationBlockedFail, ValCapitalModFail, ValChargeGroupChargeInfo, ValChargeGroupFail,
    ValChargeParentGroupFail, ValChargeParentGroupInfo, ValChargeSizeChargeInfo, ValChargeSizeFail,
    ValChargeVolumeChargeInfo, ValChargeVolumeFail, ValDroneGroupFail, ValEffectSecZoneFail, ValEffectStopperFail,
    ValFighterSquadSizeFail, ValFighterSquadSizeFighterInfo, ValItemKindFail, ValItemKindItemInfo, ValItemSecZoneFail,
    ValItemVsShipKindFail, ValMaxGroupFail, ValMaxGroupGroupInfo, ValMaxTypeFail, ValMaxTypeTypeInfo,
    ValModuleStateFail, ValModuleStateModuleInfo, ValNotLoadedItemFail, ValOverloadSkillFail, ValProjFilterFail,
    ValProjImmunityFail, ValResFail, ValRigSizeFail, ValShipKind, ValShipLimitFail, ValShipLimitItemInfo,
    ValShipStanceFail, ValSlotCountFail, ValSlotIndexFail, ValSrqFail, ValSrqSkillInfo, ValUnusableCapFail,
    ValUnusableResFail, ValUnusableSlotFail,
};

mod shared;
mod val_options;
mod val_result;
mod vast;
mod vaste_debug;
mod vaste_maintain;
mod vaste_stats;
mod vaste_validate;
mod vaste_vals;
