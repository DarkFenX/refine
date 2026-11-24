pub use fit::{FitValCmd, FitValCmdBr, FitValEnumCmdBr, FitValResult};
pub(crate) use fit::{FitValCmdGen, FitValEnumCmd};
pub use rc::val::{
    ValActivationBlockedFail, ValCapitalModFail, ValCapitalModInfo, ValChargeGroupChargeInfo, ValChargeGroupFail,
    ValChargeSizeChargeInfo, ValChargeSizeFail, ValChargeVolumeChargeInfo, ValChargeVolumeFail, ValCloakingBlockedFail,
    ValDroneGroupFail, ValDroneGroupInfo, ValEffectSecZoneEffectInfo, ValEffectSecZoneFail, ValEffectSecZoneItemInfo,
    ValEffectStopperFail, ValEffectStopperItemInfo, ValEnabled, ValFighterSquadSizeFail,
    ValFighterSquadSizeFighterInfo, ValItemKindFail, ValItemKindItemInfo, ValItemSecZoneFail, ValItemSecZoneItemInfo,
    ValItemVsShipKindFail, ValItemVsShipKindItemInfo, ValMaxGroupFail, ValMaxGroupGroupInfo, ValMaxGroupItemInfo,
    ValMaxTypeFail, ValMaxTypeItemInfo, ValMaxTypeTypeInfo, ValModuleStateFail, ValModuleStateModuleInfo,
    ValNotLoadedItemFail, ValOptions, ValOverloadSkillFail, ValOverloadSkillItemInfo, ValProjFilterFail,
    ValProjFilterItemInfo, ValProjImmunityFail, ValProjImmunityItemInfo, ValResourceFail, ValResourceItemInfo,
    ValResult, ValResultFit as FitValResultDetails, ValResultSol as SolValResultDetails, ValRigSizeFail,
    ValRigSizeItemInfo, ValShipKind, ValShipLimitFail, ValShipLimitItemInfo, ValShipStanceFail, ValSlotCountFail,
    ValSlotIndexFail, ValSlotIndexSlotInfo, ValSrqFail, ValSrqItemInfo, ValSrqSkillInfo, ValUnusableCapFail,
    ValUnusableCapItemInfo, ValUnusableResFail, ValUnusableResItemInfo, ValUnusableSlotFail,
};
pub use result_mode::ValResultMode;
pub use sol::{SolValCmd, SolValCmdBr, SolValEnumCmdBr, SolValResult};
pub(crate) use sol::{SolValCmdGen, SolValEnumCmd};

mod fit;
mod result_mode;
mod sol;

pub mod err {
    pub use crate::val::{fit::FitGetFitValError, sol::SolValEnumError};
}
