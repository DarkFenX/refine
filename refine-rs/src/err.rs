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
        ChangeStanceError, CharacterChangeError, ChargeChangeError, DroneAddError, DroneChangeError, FighterAddError,
        FighterChangeError, FitAddError, FitChangeError, FitChangeShipError, FitChangeStanceError,
        FitCharacterChangeError, FitCtlCmdError, FitGetBoosterAddError, FitGetCharacterChangeError,
        FitGetCharacterSetError, FitGetCharacterUnsetError, FitGetDroneAddError, FitGetFighterAddError,
        FitGetFitChangeError, FitGetFitRemoveError, FitGetFwEffectAddError, FitGetImplantAddError,
        FitGetModuleAddError, FitGetRigAddError, FitGetServiceAddError, FitGetSkillAddError, FitGetSubsystemAddError,
        FleetAddError, FleetChangeError, FleetGetFleetChangeError, FleetGetFleetRemoveError, FleetRemoveCmd,
        FwEffectChangeError, GetFitChangeShipError, GetFitChangeStanceError, GetFitSetShipError, GetFitSetStanceError,
        GetFitUnsetShipError, GetFitUnsetStanceError, GetItemChangeShipError, GetItemChangeStanceError,
        ImplantChangeError, ItemAddError, ItemChangeShipError, ItemChangeStanceError, ItemCharacterChangeError,
        ItemCtlError, ItemGetAutochargeChangeError, ItemGetBoosterChangeError, ItemGetCharacterChangeError,
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
