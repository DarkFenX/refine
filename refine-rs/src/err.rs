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
        AddProjEffectError, AutochargeChangeError, BackrefRenderError, BoosterChangeError, ChangeCharacterError,
        ChangeShipError, ChangeSolEnumError, ChangeStanceError, ChargeChangeError, FitAddDroneError, FitAddError,
        FitAddFighterError, FitAddModuleError, FitAddSkillError, FitChangeCharacterError, FitChangeError,
        FitChangeShipError, FitChangeStanceError, FitCtlCmdError, FitGetBoosterAddError, FitGetFitChangeError,
        FitGetFitRemoveError, FitGetImplantAddError, FitGetRigAddError, FleetAddError, FleetChangeError,
        FleetGetFleetChangeError, FleetGetFleetRemoveError, FleetRemoveCmd, GetFitAddDroneError, GetFitAddFighterError,
        GetFitAddFwEffectError, GetFitAddModuleError, GetFitAddServiceError, GetFitAddSkillError,
        GetFitAddSubsystemError, GetFitChangeCharacterError, GetFitChangeShipError, GetFitChangeStanceError,
        GetFitSetCharacterError, GetFitSetShipError, GetFitSetStanceError, GetFitUnsetCharacterError,
        GetFitUnsetShipError, GetFitUnsetStanceError, GetItemChangeCharacterError, GetItemChangeDroneError,
        GetItemChangeFighterError, GetItemChangeFwEffectError, GetItemChangeModuleError, GetItemChangeProjEffectError,
        GetItemChangeServiceError, GetItemChangeShipError, GetItemChangeSkillError, GetItemChangeStanceError,
        GetItemChangeSubsystemError, GetItemChangeSwEffectError, ImplantChangeError, ItemAddError,
        ItemChangeCharacterError, ItemChangeDroneError, ItemChangeFighterError, ItemChangeFwEffectError,
        ItemChangeModuleError, ItemChangeProjEffectError, ItemChangeServiceError, ItemChangeShipError,
        ItemChangeSkillError, ItemChangeStanceError, ItemChangeSubsystemError, ItemChangeSwEffectError, ItemCtlError,
        ItemGetAutochargeChangeError, ItemGetBoosterChangeError, ItemGetChargeChangeError, ItemGetImplantChangeError,
        ItemGetItemRemoveError, ItemGetRigChangeError, ItemRemoveError, RigChangeError,
    },
};

pub mod core {
    pub use rc::err::*;
}
