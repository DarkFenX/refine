use kinds::ValKind;
use options::ValOptionInt;
pub use options::{ValEnabled, ValOptions, ValOptionsSol};
pub(crate) use options::{ValOptionsInt, ValOptionsSolInt};
pub use result::{ValResultFit, ValResultSol};
pub(in crate::svc::vast) use validators::{
    EffectSecZoneInfo, ValFighterSquadSizeFighterStored, ValItemKindItemStored, ValModuleStateModuleStored,
    ValSrqSkillStored, get_fit_ship_rig_size, get_fit_ship_type_group, get_item_vs_ship_kind_mismatch,
    is_capital_module, is_charge_cont_group_mismatch, is_charge_group_mismatch, is_charge_size_mismatch,
    is_charge_volume_mismatch, is_drone_group_mismatch, is_rig_size_mismatch, is_ship_limit_met, is_ship_subcap,
    is_skill_level_missing,
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
mod validate_full;
mod validate_static;
mod validators;
