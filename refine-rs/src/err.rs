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
        AutochargeChangeError, BackrefRenderError, BoosterChangeError, ChangeCharacterError, ChangeShipError,
        ChangeSolEnumError, ChangeStanceError, ChargeChangeError, DroneAddError, DroneChangeError, FighterAddError,
        FighterChangeError, FitAddError, FitChangeCharacterError, FitChangeError, FitChangeShipError,
        FitChangeStanceError, FitCtlCmdError, FitGetBoosterAddError, FitGetDroneAddError, FitGetFighterAddError,
        FitGetFitChangeError, FitGetFitRemoveError, FitGetFwEffectAddError, FitGetImplantAddError,
        FitGetModuleAddError, FitGetRigAddError, FitGetServiceAddError, FitGetSkillAddError, FitGetSubsystemAddError,
        FleetAddError, FleetChangeError, FleetGetFleetChangeError, FleetGetFleetRemoveError, FleetRemoveCmd,
        FwEffectChangeError, GetFitChangeCharacterError, GetFitChangeShipError, GetFitChangeStanceError,
        GetFitSetCharacterError, GetFitSetShipError, GetFitSetStanceError, GetFitUnsetCharacterError,
        GetFitUnsetShipError, GetFitUnsetStanceError, GetItemChangeCharacterError, GetItemChangeShipError,
        GetItemChangeStanceError, ImplantChangeError, ItemAddError, ItemChangeCharacterError, ItemChangeShipError,
        ItemChangeStanceError, ItemCtlError, ItemGetAutochargeChangeError, ItemGetBoosterChangeError,
        ItemGetChargeChangeError, ItemGetDroneChangeError, ItemGetFighterChangeError, ItemGetFwEffectChangeError,
        ItemGetImplantChangeError, ItemGetItemRemoveError, ItemGetModuleChangeError, ItemGetProjEffectChangeError,
        ItemGetRigChangeError, ItemGetServiceChangeError, ItemGetSkillChangeError, ItemGetSubsystemChangeError,
        ItemGetSwEffectChangeError, ItemRemoveError, ModuleAddError, ModuleChangeError, ProjEffectAddError,
        ProjEffectChangeError, RigChangeError, ServiceChangeError, SkillAddError, SkillChangeError,
        SubsystemChangeError, SwEffectChangeError,
    },
};

pub mod core {
    pub use rc::err::*;
}
