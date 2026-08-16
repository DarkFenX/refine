#[cfg(feature = "serde")]
pub use rc::err::{ParseFitIdError, ParseFleetIdError, ParseItemIdError};

#[cfg(feature = "serde")]
pub use crate::api::ParseSolarSystemIdError;
pub use crate::{
    api::{
        AddSolError, ChangeFleetError, ChangeSolError, CtlFitChangeError, GetFitError, GetFleetError, GetItemError,
        GetSolError, RemoveItemError, RemoveSolError, SolSwitchSrcError,
    },
    ctl::{
        AddFitError, AddFleetError, AddItemEnumError, AddProjEffectError, BackrefRenderError, ChangeCharacterError,
        ChangeItemEnumError, ChangeShipError, ChangeSolEnumError, ChangeStanceError, FitAddDroneError,
        FitAddFighterError, FitAddModuleError, FitAddSkillError, FitChangeCharacterError, FitChangeError,
        FitChangeShipError, FitChangeStanceError, FitCtlCmdError, FitGetFitRemoveError, FleetChangeFleetError,
        GetFitAddBoosterError, GetFitAddDroneError, GetFitAddFighterError, GetFitAddFwEffectError,
        GetFitAddImplantError, GetFitAddModuleError, GetFitAddRigError, GetFitAddServiceError, GetFitAddSkillError,
        GetFitAddSubsystemError, GetFitChangeCharacterError, GetFitChangeShipError, GetFitChangeStanceError,
        GetFitSetCharacterError, GetFitSetShipError, GetFitSetStanceError, GetFitUnsetCharacterError,
        GetFitUnsetShipError, GetFitUnsetStanceError, GetFleetChangeFleetError, GetFleetRemoveFleetError,
        GetItemChangeAutochargeError, GetItemChangeBoosterError, GetItemChangeCharacterError, GetItemChangeChargeError,
        GetItemChangeDroneError, GetItemChangeFighterError, GetItemChangeFwEffectError, GetItemChangeImplantError,
        GetItemChangeModuleError, GetItemChangeProjEffectError, GetItemChangeRigError, GetItemChangeServiceError,
        GetItemChangeShipError, GetItemChangeSkillError, GetItemChangeStanceError, GetItemChangeSubsystemError,
        GetItemChangeSwEffectError, GetItemRemoveItemError, ItemChangeAutochargeError, ItemChangeBoosterError,
        ItemChangeCharacterError, ItemChangeChargeError, ItemChangeDroneError, ItemChangeFighterError,
        ItemChangeFwEffectError, ItemChangeImplantError, ItemChangeModuleError, ItemChangeProjEffectError,
        ItemChangeRigError, ItemChangeServiceError, ItemChangeShipError, ItemChangeSkillError, ItemChangeStanceError,
        ItemChangeSubsystemError, ItemChangeSwEffectError, ItemRemoveItemError, SolCtlFitChangeError,
    },
};

pub mod core {
    pub use rc::err::*;
}
