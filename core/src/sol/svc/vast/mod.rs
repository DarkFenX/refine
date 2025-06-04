use cache::ValCache;
pub(in crate::sol) use options::{IntSolValOptions, IntValOptions};
pub use options::{SolValOptions, ValOption, ValOptions};
pub use result::ValResult;
pub(in crate::sol::svc) use vast::Vast;
use vast::VastFitData;
use vaste_stats_fit::{StatRes, StatSlot};
pub use vaste_vals_fit::{
    ValActivationBlockedFail, ValCapitalModFail, ValChargeGroupChargeInfo, ValChargeGroupFail, ValChargeSizeChargeInfo,
    ValChargeSizeFail, ValChargeVolumeChargeInfo, ValChargeVolumeFail, ValDroneGroupFail, ValEffectImmunityFail,
    ValEffectStopperFail, ValFighterSquadSizeFail, ValFighterSquadSizeFighterInfo, ValItemKindFail,
    ValItemKindItemInfo, ValItemVsShipKindFail, ValMaxGroupFail, ValMaxGroupGroupInfo, ValMaxTypeFail,
    ValMaxTypeTypeInfo, ValModuleStateFail, ValModuleStateModuleInfo, ValNotLoadedItemFail, ValOverloadSkillFail,
    ValResFail, ValRigSizeFail, ValSecZoneFail, ValShipKind, ValShipLimitFail, ValShipLimitItemInfo, ValShipStanceFail,
    ValSlotCountFail, ValSlotIndexFail, ValSrqFail, ValSrqSkillInfo, ValUnusableResFail, ValUnusableSlotFail,
};
use vaste_vals_fit::{ValChargeGroupFailCache, ValChargeSizeFailCache, ValChargeVolumeFailCache};

mod cache;
mod options;
mod result;
mod vast;
mod vaste_debug;
mod vaste_maintain;
mod vaste_stats_fit;
mod vaste_validate;
mod vaste_vals_fit;
