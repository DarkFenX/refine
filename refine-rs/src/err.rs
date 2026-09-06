#[cfg(feature = "serde")]
pub use rc::err::{FitIdParseError, FleetIdParseError, ItemIdParseError};

#[cfg(feature = "serde")]
pub use crate::api::SolarSystemIdParseError;
pub use crate::{
    api::{
        FitGetError, FitHybridBatchError, FleetGetError, ItemGetError, SolAddError, SolFittingAppError, SolGetError,
        SolHybridBatchError, SolRemoveError, SolSwitchSrcError,
    },
    ctl::{
        AutochargeChangeError, BoosterChangeError, CharacterChangeError, ChargeChangeError, DroneAddError,
        DroneChangeError, FighterAddError, FighterChangeError, FitAddError, FitChangeEnumError, FitChangeError,
        FitCharacterChangeError, FitGetBoosterAddError, FitGetCharacterChangeError, FitGetCharacterSetError,
        FitGetCharacterUnsetError, FitGetDroneAddError, FitGetFighterAddError, FitGetFitChangeError,
        FitGetFitRemoveError, FitGetFwEffectAddError, FitGetImplantAddError, FitGetItemAddAutoError,
        FitGetModuleAddError, FitGetRigAddError, FitGetServiceAddError, FitGetShipChangeError, FitGetShipSetError,
        FitGetShipUnsetError, FitGetSkillAddError, FitGetStanceChangeError, FitGetStanceSetError,
        FitGetStanceUnsetError, FitGetSubsystemAddError, FitShipChangeError, FitStanceChangeError, FleetAddError,
        FleetChangeError, FleetGetFleetChangeError, FleetGetFleetRemoveError, FleetRemoveCmd, FwEffectChangeError,
        ImplantChangeError, ItemAddAutoError, ItemAddEnumError, ItemChangeEnumError, ItemCharacterChangeError,
        ItemGetAutochargeChangeError, ItemGetBoosterChangeError, ItemGetCharacterChangeError, ItemGetChargeChangeError,
        ItemGetDroneChangeError, ItemGetFighterChangeError, ItemGetFwEffectChangeError, ItemGetImplantChangeError,
        ItemGetItemRemoveError, ItemGetModuleChangeError, ItemGetProjEffectChangeError, ItemGetRigChangeError,
        ItemGetServiceChangeError, ItemGetShipChangeError, ItemGetSkillChangeError, ItemGetStanceChangeError,
        ItemGetSubsystemChangeError, ItemGetSwEffectChangeError, ItemRemoveError, ItemShipChangeError,
        ItemStanceChangeError, ModuleAddError, ModuleChangeError, ProjEffectAddError, ProjEffectChangeError,
        RigChangeError, ServiceChangeError, ShipChangeError, SkillAddError, SkillChangeError, SolChangeEnumError,
        StanceChangeError, SubsystemChangeError, SwEffectChangeError,
    },
    hyb::{FitHybridError, SolHybridError},
    info::{FitGetFitInfoError, FitInfoEnumError, FleetGetFleetInfoError, ItemGetItemInfoError, SolInfoEnumError},
    shared::BrResolveError,
};

pub mod core {
    pub use rc::err::*;
}
