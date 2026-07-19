#[cfg(feature = "serde")]
pub use rc::err::{ParseFitIdError, ParseFleetIdError, ParseItemIdError};

#[cfg(feature = "serde")]
pub use crate::api::ParseSolarSystemIdError;
pub use crate::{
    api::{
        AddSolError, ChangeFitError, ChangeFleetError, ChangeSolError, GetFitError, GetFleetError, GetItemError,
        GetSolError, RemoveItemError, RemoveSolError, SolSwitchSrcError,
    },
    cmd::{
        AddFitError, AddFleetError, AddItemEnumError, AddProjEffectError, BackrefRenderError, ChangeCharacterError,
        ChangeFitEnumError, ChangeItemEnumError, ChangeShipError, ChangeSolEnumError, ChangeStanceError,
        FitAddDroneError, FitAddFighterError, FitAddModuleError, FitAddSkillError, FitChangeCharacterError,
        FitChangeFitError, FitChangeShipError, FitChangeStanceError, FleetChangeFleetError, GetFitAddBoosterError,
        GetFitAddDroneError, GetFitAddFighterError, GetFitAddFwEffectError, GetFitAddImplantError,
        GetFitAddModuleError, GetFitAddRigError, GetFitAddServiceError, GetFitAddSkillError, GetFitAddSubsystemError,
        GetFitChangeCharacterError, GetFitChangeFitError, GetFitChangeShipError, GetFitChangeStanceError,
        GetFitRemoveFitError, GetFitSetCharacterError, GetFitSetShipError, GetFitSetStanceError,
        GetFitUnsetCharacterError, GetFitUnsetShipError, GetFitUnsetStanceError, GetFleetChangeFleetError,
        GetFleetRemoveFleetError, GetItemChangeAutochargeError, GetItemChangeBoosterError, GetItemChangeCharacterError,
        GetItemChangeChargeError, GetItemChangeDroneError, GetItemChangeFighterError, GetItemChangeFwEffectError,
        GetItemChangeImplantError, GetItemChangeModuleError, GetItemChangeProjEffectError, GetItemChangeRigError,
        GetItemChangeServiceError, GetItemChangeShipError, GetItemChangeSkillError, GetItemChangeStanceError,
        GetItemChangeSubsystemError, GetItemChangeSwEffectError, GetItemRemoveItemError, ItemChangeAutochargeError,
        ItemChangeBoosterError, ItemChangeCharacterError, ItemChangeChargeError, ItemChangeDroneError,
        ItemChangeFighterError, ItemChangeFwEffectError, ItemChangeImplantError, ItemChangeModuleError,
        ItemChangeProjEffectError, ItemChangeRigError, ItemChangeServiceError, ItemChangeShipError,
        ItemChangeSkillError, ItemChangeStanceError, ItemChangeSubsystemError, ItemChangeSwEffectError,
        ItemRemoveItemError,
    },
};

pub mod core {
    pub use rc::err::*;
}
