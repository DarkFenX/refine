pub(crate) use stats::StatCapSimStaggerInt;
pub use stats::{
    StatCapBlcRegen, StatCapBlcRegenOptions, StatCapBlcSrcKinds, StatCapSim, StatCapSimStagger, StatDmg,
    StatDmgApplied, StatDmgEntry, StatDmgEntryApplied, StatDmgEntryBreacher, StatDmgItemKinds, StatEhp, StatEhpLayer,
    StatErps, StatErpsLayer, StatErpsLayerRegen, StatHp, StatHpLayer, StatInJam, StatMining, StatMiningEntry,
    StatMiningItemKinds, StatNeutItemKinds, StatOutRepItemKinds, StatOutReps, StatResists, StatResistsLayer,
    StatResource, StatRps, StatRpsLayer, StatRpsLayerRegen, StatSensors, StatSensorsKind, StatSlot, StatTimeOptions,
    StatTimeOptionsBurst, StatTimeOptionsSim,
};
use val_options::ValOptionInt;
pub use val_options::{ValOption, ValOptionEnabledOptions, ValOptions, ValOptionsSol};
pub(crate) use val_options::{ValOptionsInt, ValOptionsSolInt};
pub use val_result::{ValResultFit, ValResultSol};
pub use validators::{
    ValActivationBlockedFail, ValCapitalModFail, ValChargeGroupChargeInfo, ValChargeGroupFail,
    ValChargeParentGroupFail, ValChargeParentGroupInfo, ValChargeSizeChargeInfo, ValChargeSizeFail,
    ValChargeVolumeChargeInfo, ValChargeVolumeFail, ValDroneGroupFail, ValEffectSecZoneFail, ValEffectStopperFail,
    ValFighterSquadSizeFail, ValFighterSquadSizeFighterInfo, ValItemKindFail, ValItemKindItemInfo, ValItemSecZoneFail,
    ValItemVsShipKindFail, ValMaxGroupFail, ValMaxGroupGroupInfo, ValMaxTypeFail, ValMaxTypeTypeInfo,
    ValModuleStateFail, ValModuleStateModuleInfo, ValNotLoadedItemFail, ValOverloadSkillFail, ValProjFilterFail,
    ValProjImmunityFail, ValResourceFail, ValRigSizeFail, ValShipKind, ValShipLimitFail, ValShipLimitItemInfo,
    ValShipStanceFail, ValSlotCountFail, ValSlotIndexFail, ValSrqFail, ValSrqSkillInfo, ValUnusableCapFail,
    ValUnusableResFail, ValUnusableSlotFail,
};
pub(in crate::svc) use vast::Vast;
use vast::VastFitData;

mod aggr;
mod maintain;
mod shared;
mod stats;
mod val_options;
mod val_result;
mod validators;
mod vast;
mod vaste_debug;
mod vaste_validate;
