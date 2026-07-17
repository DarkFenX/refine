pub use stats::{
    StatCapBlcNosfs, StatCapBlcRegen, StatCapBlcSrcKinds, StatCapSim, StatCapSimStagger, StatDmg, StatDmgApplied,
    StatDmgEntry, StatDmgEntryApplied, StatDmgEntryBreacher, StatDmgItemKinds, StatEhp, StatEhpLayer, StatErps,
    StatErpsLayer, StatErpsLayerRegen, StatHp, StatHpLayer, StatInJam, StatJump, StatJumpConduit, StatJumpPassenger,
    StatJumpPortal, StatJumpRange, StatJumpSelf, StatMining, StatMiningEntry, StatMiningItemKinds, StatNeutItemKinds,
    StatOutRepItemKinds, StatOutReps, StatResists, StatResistsLayer, StatResource, StatRps, StatRpsLayer,
    StatRpsLayerRegen, StatSensors, StatSensorsKind, StatSlot, StatTimeOptions, StatTimeOptionsBurst,
    StatTimeOptionsSim,
};
pub(crate) use stats::{StatCapBlcNosfsOptionsInt, StatCapBlcSrcKindsInt, StatCapSimStaggerInt};
use val_options::ValOptionInt;
pub use val_options::{ValEnabled, ValOptions, ValOptionsSol};
pub(crate) use val_options::{ValOptionsInt, ValOptionsSolInt};
pub use val_result::{ValResultFit, ValResultSol};
pub use validators::{
    ValActivationBlockedFail, ValCapitalModFail, ValChargeGroupChargeInfo, ValChargeGroupFail,
    ValChargeParentGroupFail, ValChargeParentGroupInfo, ValChargeSizeChargeInfo, ValChargeSizeFail,
    ValChargeVolumeChargeInfo, ValChargeVolumeFail, ValCloakingBlockedFail, ValDroneGroupFail, ValEffectSecZoneFail,
    ValEffectStopperFail, ValFighterSquadSizeFail, ValFighterSquadSizeFighterInfo, ValItemKindFail,
    ValItemKindItemInfo, ValItemSecZoneFail, ValItemVsShipKindFail, ValMaxGroupFail, ValMaxGroupGroupInfo,
    ValMaxTypeFail, ValMaxTypeTypeInfo, ValModuleStateFail, ValModuleStateModuleInfo, ValNotLoadedItemFail,
    ValOverloadSkillFail, ValProjFilterFail, ValProjImmunityFail, ValResourceFail, ValRigSizeFail, ValShipKind,
    ValShipLimitFail, ValShipLimitItemInfo, ValShipStanceFail, ValSlotCountFail, ValSlotIndexFail, ValSrqFail,
    ValSrqSkillInfo, ValUnusableCapFail, ValUnusableResFail, ValUnusableSlotFail,
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
