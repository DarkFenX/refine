use kinds::ValKind;
use options::ValOptionInt;
pub use options::{ValEnabled, ValOptions, ValOptionsSol};
pub(crate) use options::{ValOptionsInt, ValOptionsSolInt};
pub use result::{ValResultFit, ValResultSol};
pub(in crate::svc::vast) use validators::{
    EffectSecZoneInfo, ValFighterSquadSizeFighterStored, ValItemKindItemStored, ValModuleStateModuleStored,
    ValSrqSkillStored, is_charge_cont_group_mismatch, is_charge_group_mismatch, is_charge_size_mismatch,
    is_charge_volume_mismatch,
};
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

mod kinds;
mod options;
mod result;
mod validate;
mod validators;
