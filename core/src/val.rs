//! Validation-related exports
pub use crate::{
    misc::ItemKind as ValItemKind,
    svc::vast::{
        ValActivationBlockedFail, ValCapUseFail, ValCapitalModFail, ValChargeGroupChargeInfo, ValChargeGroupFail,
        ValChargeParentGroupFail, ValChargeParentGroupInfo, ValChargeSizeChargeInfo, ValChargeSizeFail,
        ValChargeVolumeChargeInfo, ValChargeVolumeFail, ValDroneGroupFail, ValEffectStopperFail,
        ValFighterSquadSizeFail, ValFighterSquadSizeFighterInfo, ValItemKindFail, ValItemKindItemInfo,
        ValItemVsShipKindFail, ValMaxGroupFail, ValMaxGroupGroupInfo, ValMaxTypeFail, ValMaxTypeTypeInfo,
        ValModuleStateFail, ValModuleStateModuleInfo, ValNotLoadedItemFail, ValOption, ValOptions, ValOptionsSol,
        ValOverloadSkillFail, ValProjFilterFail, ValProjImmunityFail, ValResFail, ValResultFit, ValResultSol,
        ValRigSizeFail, ValSecZoneFail, ValShipKind, ValShipLimitFail, ValShipLimitItemInfo, ValShipStanceFail,
        ValSlotCountFail, ValSlotIndexFail, ValSrqFail, ValSrqSkillInfo, ValUnusableResFail, ValUnusableSlotFail,
    },
};
