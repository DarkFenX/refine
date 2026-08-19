pub use stats::{
    StatAgilityError, StatCapBlcNosfs, StatCapBlcRegen, StatCapBlcSrcKinds, StatCapSim, StatCapSimStagger,
    StatCritOptions, StatDmg, StatDmgApplied, StatDmgEntry, StatDmgEntryApplied, StatDmgEntryBreacher,
    StatDmgItemKinds, StatEhp, StatEhpLayer, StatErps, StatErpsLayer, StatErpsLayerRegen, StatHp, StatHpLayer,
    StatInJam, StatItemChargeOptions, StatJump, StatJumpConduit, StatJumpError, StatJumpPassenger, StatJumpPortal,
    StatJumpRange, StatJumpSelf, StatMaxWarpRangeError, StatMining, StatMiningEntry, StatMiningItemKinds,
    StatMiningResourceKind, StatNeutItemKinds, StatOutRepItemKinds, StatOutReps, StatProbingSizeError, StatResists,
    StatResistsLayer, StatResource, StatRps, StatRpsLayer, StatRpsLayerRegen, StatSensors, StatSensorsKind, StatSlot,
    StatTimeOptions, StatTimeOptionsBurst, StatTimeOptionsSim, StatWarpSpeedError,
};
pub(crate) use stats::{StatCapBlcNosfsOptionsInt, StatCapBlcSrcKindsInt, StatCapSimStaggerInt};
pub use val::{
    ValActivationBlockedFail, ValCapitalModFail, ValCapitalModInfo, ValChargeGroupChargeInfo, ValChargeGroupFail,
    ValChargeParentGroupChargeInfo, ValChargeParentGroupFail, ValChargeSizeChargeInfo, ValChargeSizeFail,
    ValChargeVolumeChargeInfo, ValChargeVolumeFail, ValCloakingBlockedFail, ValDroneGroupFail, ValDroneGroupInfo,
    ValEffectSecZoneEffectInfo, ValEffectSecZoneFail, ValEffectSecZoneItemInfo, ValEffectStopperFail,
    ValEffectStopperItemInfo, ValEnabled, ValFighterSquadSizeFail, ValFighterSquadSizeFighterInfo, ValItemKindFail,
    ValItemKindItemInfo, ValItemSecZoneFail, ValItemSecZoneItemInfo, ValItemVsShipKindFail, ValItemVsShipKindItemInfo,
    ValMaxGroupFail, ValMaxGroupGroupInfo, ValMaxGroupItemInfo, ValMaxTypeFail, ValMaxTypeItemInfo, ValMaxTypeTypeInfo,
    ValModuleStateFail, ValModuleStateModuleInfo, ValNotLoadedItemFail, ValOptions, ValOptionsSol,
    ValOverloadSkillFail, ValOverloadSkillItemInfo, ValProjFilterFail, ValProjFilterItemInfo, ValProjImmunityFail,
    ValProjImmunityItemInfo, ValResourceFail, ValResourceItemInfo, ValResultFit, ValResultSol, ValRigSizeFail,
    ValRigSizeItemInfo, ValShipKind, ValShipLimitFail, ValShipLimitItemInfo, ValShipStanceFail, ValSlotCountFail,
    ValSlotIndexFail, ValSlotIndexSlotInfo, ValSrqFail, ValSrqItemInfo, ValSrqSkillInfo, ValUnusableCapFail,
    ValUnusableCapItemInfo, ValUnusableResFail, ValUnusableResItemInfo, ValUnusableSlotFail,
};
pub(crate) use val::{ValOptionsInt, ValOptionsSolInt};
pub(in crate::svc) use vast::Vast;
use vast::VastFitData;

mod aggr;
mod maintain;
mod shared;
mod stats;
mod val;
mod vast;
mod vaste_debug;
