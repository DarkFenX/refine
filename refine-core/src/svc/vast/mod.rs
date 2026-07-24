pub use stats::{
    AgilityStatError, JumpStatError, MaxWarpRangeStatError, ProbingSizeStatError, StatCapBlcNosfs, StatCapBlcRegen,
    StatCapBlcSrcKinds, StatCapSim, StatCapSimStagger, StatDmg, StatDmgApplied, StatDmgEntry, StatDmgEntryApplied,
    StatDmgEntryBreacher, StatDmgItemKinds, StatEhp, StatEhpLayer, StatErps, StatErpsLayer, StatErpsLayerRegen, StatHp,
    StatHpLayer, StatInJam, StatJump, StatJumpConduit, StatJumpPassenger, StatJumpPortal, StatJumpRange, StatJumpSelf,
    StatMining, StatMiningEntry, StatMiningItemKinds, StatNeutItemKinds, StatOutRepItemKinds, StatOutReps, StatResists,
    StatResistsLayer, StatResource, StatRps, StatRpsLayer, StatRpsLayerRegen, StatSensors, StatSensorsKind, StatSlot,
    StatTimeOptions, StatTimeOptionsBurst, StatTimeOptionsSim, WarpSpeedStatError,
};
pub(crate) use stats::{StatCapBlcNosfsOptionsInt, StatCapBlcSrcKindsInt, StatCapSimStaggerInt};
use val_options::ValOptionInt;
pub use val_options::{ValEnabled, ValOptions, ValOptionsSol};
pub(crate) use val_options::{ValOptionsInt, ValOptionsSolInt};
pub use val_result::{ValResultFit, ValResultSol};
pub use validators::{
    ValActivationBlockedFail, ValCapitalModFail, ValCapitalModInfo, ValChargeGroupChargeInfo, ValChargeGroupFail,
    ValChargeParentGroupChargeInfo, ValChargeParentGroupFail, ValChargeSizeChargeInfo, ValChargeSizeFail,
    ValChargeVolumeChargeInfo, ValChargeVolumeFail, ValCloakingBlockedFail, ValDroneGroupFail, ValDroneGroupInfo,
    ValEffectSecZoneEffectInfo, ValEffectSecZoneFail, ValEffectSecZoneItemInfo, ValEffectStopperFail,
    ValEffectStopperItemInfo, ValFighterSquadSizeFail, ValFighterSquadSizeFighterInfo, ValItemKindFail,
    ValItemKindItemInfo, ValItemSecZoneFail, ValItemSecZoneItemInfo, ValItemVsShipKindFail, ValItemVsShipKindItemInfo,
    ValMaxGroupFail, ValMaxGroupGroupInfo, ValMaxGroupItemInfo, ValMaxTypeFail, ValMaxTypeItemInfo, ValMaxTypeTypeInfo,
    ValModuleStateFail, ValModuleStateModuleInfo, ValNotLoadedItemFail, ValOverloadSkillFail, ValOverloadSkillItemInfo,
    ValProjFilterFail, ValProjFilterItemInfo, ValProjImmunityFail, ValProjImmunityItemInfo, ValResourceFail,
    ValResourceItemInfo, ValRigSizeFail, ValRigSizeItemInfo, ValShipKind, ValShipLimitFail, ValShipLimitItemInfo,
    ValShipStanceFail, ValSlotCountFail, ValSlotIndexFail, ValSlotIndexSlotInfo, ValSrqFail, ValSrqItemInfo,
    ValSrqSkillInfo, ValUnusableCapFail, ValUnusableCapItemInfo, ValUnusableResFail, ValUnusableResItemInfo,
    ValUnusableSlotFail,
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
