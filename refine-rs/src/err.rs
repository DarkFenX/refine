#[cfg(feature = "serde")]
pub use rc::err::{ParseFitIdError, ParseFleetIdError, ParseItemIdError};

#[cfg(feature = "serde")]
pub use crate::api::ParseSolarSystemIdError;
pub use crate::{
    api::{
        FitChangeEnumFitInfoError, FitGetError, FitHybridBatchError, FleetGetError, ItemGetError, SolAddError,
        SolChangeEnumSolInfoError, SolGetError, SolHybridBatchError, SolRemoveError, SolSwitchSrcError,
    },
    ctl::{
        AutochargeChangeError, BoosterChangeError, CharacterChangeError, ChargeChangeError, DroneAddError,
        DroneChangeError, FighterAddError, FighterChangeError, FitAddError, FitChangeEnumError, FitChangeError,
        FitCharacterChangeError, FitGetBoosterAddError, FitGetCharacterChangeError, FitGetCharacterSetError,
        FitGetCharacterUnsetError, FitGetDroneAddError, FitGetFighterAddError, FitGetFitChangeError,
        FitGetFitRemoveError, FitGetFwEffectAddError, FitGetImplantAddError, FitGetModuleAddError, FitGetRigAddError,
        FitGetServiceAddError, FitGetShipChangeError, FitGetShipSetError, FitGetShipUnsetError, FitGetSkillAddError,
        FitGetStanceChangeError, FitGetStanceSetError, FitGetStanceUnsetError, FitGetSubsystemAddError,
        FitShipChangeError, FitStanceChangeError, FleetAddError, FleetChangeError, FleetGetFleetChangeError,
        FleetGetFleetRemoveError, FleetRemoveCmd, FwEffectChangeError, ImplantChangeError, ItemAddEnumError,
        ItemChangeEnumError, ItemCharacterChangeError, ItemGetAutochargeChangeError, ItemGetBoosterChangeError,
        ItemGetCharacterChangeError, ItemGetChargeChangeError, ItemGetDroneChangeError, ItemGetFighterChangeError,
        ItemGetFwEffectChangeError, ItemGetImplantChangeError, ItemGetItemRemoveError, ItemGetModuleChangeError,
        ItemGetProjEffectChangeError, ItemGetRigChangeError, ItemGetServiceChangeError, ItemGetShipChangeError,
        ItemGetSkillChangeError, ItemGetStanceChangeError, ItemGetSubsystemChangeError, ItemGetSwEffectChangeError,
        ItemRemoveError, ItemShipChangeError, ItemStanceChangeError, ModuleAddError, ModuleChangeError,
        ProjEffectAddError, ProjEffectChangeError, RigChangeError, ServiceChangeError, ShipChangeError, SkillAddError,
        SkillChangeError, SolChangeEnumError, StanceChangeError, SubsystemChangeError, SwEffectChangeError,
    },
    hybrid::{FitHybridError, SolHybridError},
    shared::BrResolveError,
};

pub mod core {
    pub use rc::err::*;
}
