pub use shared::{
    StatDmg, StatDmgApplied, StatDmgBreacher, StatDmgItemKinds, StatMining, StatMiningItemKinds, StatNeutItemKinds,
    StatOutgoingJam, StatRemoteRepItemKinds, StatSensor, StatSensorKind, StatTank, StatTankRegen,
};
pub use val_options::{ValOption, ValOptions, ValOptionsSol};
pub(crate) use val_options::{ValOptionsInt, ValOptionsSolInt};
pub use val_result::{ValResultFit, ValResultSol};
pub(in crate::svc) use vast::Vast;
use vast::VastFitData;
pub use vaste_stats_fit::{StatRes, StatSlot};
pub(crate) use vaste_stats_item::StatCapSimStaggerInt;
pub use vaste_stats_item::{
    StatCapConsumerOptions, StatCapRegenOptions, StatCapSim, StatCapSimStagger, StatCapSrcKinds, StatLayerEhp,
    StatLayerErps, StatLayerErpsRegen, StatLayerHp, StatLayerRps, StatLayerRpsRegen,
};
pub use vaste_vals::{
    ValActivationBlockedFail, ValCapitalModFail, ValChargeGroupChargeInfo, ValChargeGroupFail,
    ValChargeParentGroupFail, ValChargeParentGroupInfo, ValChargeSizeChargeInfo, ValChargeSizeFail,
    ValChargeVolumeChargeInfo, ValChargeVolumeFail, ValDroneGroupFail, ValEffectStopperFail, ValFighterSquadSizeFail,
    ValFighterSquadSizeFighterInfo, ValItemKindFail, ValItemKindItemInfo, ValItemVsShipKindFail, ValMaxGroupFail,
    ValMaxGroupGroupInfo, ValMaxTypeFail, ValMaxTypeTypeInfo, ValModuleStateFail, ValModuleStateModuleInfo,
    ValNotLoadedItemFail, ValOverloadSkillFail, ValProjImmunityFail, ValResFail, ValRigSizeFail, ValSecZoneFail,
    ValShipKind, ValShipLimitFail, ValShipLimitItemInfo, ValShipStanceFail, ValSlotCountFail, ValSlotIndexFail,
    ValSrqFail, ValSrqSkillInfo, ValUnusableResFail, ValUnusableSlotFail,
};

mod shared;
mod val_options;
mod val_result;
mod vast;
mod vaste_debug;
mod vaste_maintain;
mod vaste_stats_fit;
mod vaste_stats_item;
mod vaste_validate;
mod vaste_vals;
