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
        FitGetFitRemoveError, FitGetImplantAddError, FleetAddError, FleetChangeError, FleetGetFleetChangeError,
        FleetGetFleetRemoveError, FleetRemoveCmd, GetFitAddDroneError, GetFitAddFighterError, GetFitAddFwEffectError,
        GetFitAddModuleError, GetFitAddRigError, GetFitAddServiceError, GetFitAddSkillError, GetFitAddSubsystemError,
        GetFitChangeCharacterError, GetFitChangeShipError, GetFitChangeStanceError, GetFitSetCharacterError,
        GetFitSetShipError, GetFitSetStanceError, GetFitUnsetCharacterError, GetFitUnsetShipError,
        GetFitUnsetStanceError, GetItemChangeCharacterError, GetItemChangeDroneError, GetItemChangeFighterError,
        GetItemChangeFwEffectError, GetItemChangeImplantError, GetItemChangeModuleError, GetItemChangeProjEffectError,
        GetItemChangeRigError, GetItemChangeServiceError, GetItemChangeShipError, GetItemChangeSkillError,
        GetItemChangeStanceError, GetItemChangeSubsystemError, GetItemChangeSwEffectError, ItemAddError,
        ItemChangeCharacterError, ItemChangeDroneError, ItemChangeFighterError, ItemChangeFwEffectError,
        ItemChangeImplantError, ItemChangeModuleError, ItemChangeProjEffectError, ItemChangeRigError,
        ItemChangeServiceError, ItemChangeShipError, ItemChangeSkillError, ItemChangeStanceError,
        ItemChangeSubsystemError, ItemChangeSwEffectError, ItemCtlError, ItemGetAutochargeChangeError,
        ItemGetBoosterChangeError, ItemGetChargeChangeError, ItemGetItemRemoveError, ItemRemoveError,
    },
};

pub mod core {
    pub use rc::err::*;
}
