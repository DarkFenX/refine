pub use options::{ValOption, ValOptions, ValOptionsSol};
pub(crate) use options::{ValOptionsInt, ValOptionsSolInt};
pub use result::{ValResultFit, ValResultSol};
pub use shared::{
    StatDmg, StatDmgApplied, StatDmgBreacher, StatDmgItemKinds, StatNeutItemKinds, StatRemoteRepItemKinds, StatSensor,
    StatSensorKind, StatTank,
};
pub(in crate::svc) use vast::Vast;
use vast::VastFitData;
pub use vaste_stats_fit::{StatRes, StatSlot};
pub use vaste_stats_item::{StatCapSrcKinds, StatLayerEhp, StatLayerErps, StatLayerHp, StatLayerRps};
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

mod options;
mod result;
mod shared;
mod vast;
mod vaste_debug;
mod vaste_maintain;
mod vaste_stats_fit;
mod vaste_stats_item;
mod vaste_validate;
mod vaste_vals;
