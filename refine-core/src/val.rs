//! Validation-related exports
pub use crate::{
    misc::DetectedItemKind,
    svc::vast::{
        ValActivationBlockedFail, ValCapitalModFail, ValCapitalModInfo, ValChargeGroupChargeInfo, ValChargeGroupFail,
        ValChargeParentGroupChargeInfo, ValChargeParentGroupFail, ValChargeSizeChargeInfo, ValChargeSizeFail,
        ValChargeVolumeChargeInfo, ValChargeVolumeFail, ValCloakingBlockedFail, ValDroneGroupFail, ValDroneGroupInfo,
        ValEffectSecZoneFail, ValEffectStopperFail, ValEffectStopperItemInfo, ValEnabled, ValFighterSquadSizeFail,
        ValFighterSquadSizeFighterInfo, ValItemKindFail, ValItemKindItemInfo, ValItemSecZoneFail,
        ValItemVsShipKindFail, ValItemVsShipKindItemInfo, ValMaxGroupFail, ValMaxGroupGroupInfo, ValMaxGroupItemInfo,
        ValMaxTypeFail, ValMaxTypeItemInfo, ValMaxTypeTypeInfo, ValModuleStateFail, ValModuleStateModuleInfo,
        ValNotLoadedItemFail, ValOptions, ValOptionsSol, ValOverloadSkillFail, ValOverloadSkillItemInfo,
        ValProjFilterFail, ValProjFilterItemInfo, ValProjImmunityFail, ValProjImmunityItemInfo, ValResourceFail,
        ValResourceItemInfo, ValResultFit, ValResultSol, ValRigSizeFail, ValShipKind, ValShipLimitFail,
        ValShipLimitItemInfo, ValShipStanceFail, ValSlotCountFail, ValSlotIndexFail, ValSrqFail, ValSrqSkillInfo,
        ValUnusableCapFail, ValUnusableResFail, ValUnusableSlotFail,
    },
};
