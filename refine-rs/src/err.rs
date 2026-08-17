#[cfg(feature = "serde")]
pub use rc::err::{ParseFitIdError, ParseFleetIdError, ParseItemIdError};

#[cfg(feature = "serde")]
pub use crate::api::ParseSolarSystemIdError;
pub use crate::{
    api::{
        AddSolError, ChangeSolError, CtlFitChangeError, GetFitError, GetFleetError, GetItemError, GetSolError,
        RemoveItemError, RemoveSolError, SolSwitchSrcError,
    },
    ctl::{
        AutochargeChangeError, BackrefRenderError, BoosterChangeError, ChangeShipError, ChangeSolEnumError,
        CharacterChangeError, ChargeChangeError, DroneAddError, DroneChangeError, FighterAddError, FighterChangeError,
        FitAddError, FitChangeError, FitChangeShipError, FitCharacterChangeError, FitCtlCmdError,
        FitGetBoosterAddError, FitGetCharacterChangeError, FitGetCharacterSetError, FitGetCharacterUnsetError,
        FitGetDroneAddError, FitGetFighterAddError, FitGetFitChangeError, FitGetFitRemoveError, FitGetFwEffectAddError,
        FitGetImplantAddError, FitGetModuleAddError, FitGetRigAddError, FitGetServiceAddError, FitGetSkillAddError,
        FitGetStanceChangeError, FitGetStanceSetError, FitGetStanceUnsetError, FitGetSubsystemAddError,
        FitStanceChangeError, FleetAddError, FleetChangeError, FleetGetFleetChangeError, FleetGetFleetRemoveError,
        FleetRemoveCmd, FwEffectChangeError, GetFitChangeShipError, GetFitSetShipError, GetFitUnsetShipError,
        GetItemChangeShipError, ImplantChangeError, ItemAddError, ItemChangeShipError, ItemCharacterChangeError,
        ItemCtlError, ItemGetAutochargeChangeError, ItemGetBoosterChangeError, ItemGetCharacterChangeError,
        ItemGetChargeChangeError, ItemGetDroneChangeError, ItemGetFighterChangeError, ItemGetFwEffectChangeError,
        ItemGetImplantChangeError, ItemGetItemRemoveError, ItemGetModuleChangeError, ItemGetProjEffectChangeError,
        ItemGetRigChangeError, ItemGetServiceChangeError, ItemGetSkillChangeError, ItemGetStanceChangeError,
        ItemGetSubsystemChangeError, ItemGetSwEffectChangeError, ItemRemoveError, ItemStanceChangeError,
        ModuleAddError, ModuleChangeError, ProjEffectAddError, ProjEffectChangeError, RigChangeError,
        ServiceChangeError, SkillAddError, SkillChangeError, StanceChangeError, SubsystemChangeError,
        SwEffectChangeError,
    },
};

pub mod core {
    pub use rc::err::*;
}
