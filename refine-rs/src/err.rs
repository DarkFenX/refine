pub use crate::{
    cmd::{
        AddFitError, AddFleetError, AddItemEnumError, AddProjEffectError, BackrefRenderError, ChangeCharacterError,
        ChangeFitEnumError, ChangeItemEnumError, ChangeShipError, ChangeSolEnumError, ChangeStanceError,
        FitAddDroneError, FitAddFighterError, FitAddModuleError, FitAddSkillError, FitChangeCharacterError,
        FitChangeFitError, FitChangeShipError, FitChangeStanceError, FleetChangeFleetError, GetFitAddBoosterError,
        GetFitAddDroneError, GetFitAddFighterError, GetFitAddFwEffectError, GetFitAddImplantError,
        GetFitAddModuleError, GetFitAddRigError, GetFitAddServiceError, GetFitAddSkillError,
        GetFitChangeCharacterError, GetFitChangeFitError, GetFitChangeShipError, GetFitChangeStanceError,
        GetFitRemoveFitError, GetFitSetCharacterError, GetFitSetShipError, GetFitSetStanceError,
        GetFitUnsetCharacterError, GetFitUnsetShipError, GetFitUnsetStanceError, GetFleetChangeFleetError,
        GetFleetRemoveFleetError, GetItemChangeAutochargeError, GetItemChangeBoosterError, GetItemChangeCharacterError,
        GetItemChangeChargeError, GetItemChangeDroneError, GetItemChangeFighterError, GetItemChangeFwEffectError,
        GetItemChangeImplantError, GetItemChangeModuleError, GetItemChangeProjEffectError, GetItemChangeRigError,
        GetItemChangeServiceError, GetItemChangeShipError, GetItemChangeSkillError, GetItemChangeStanceError,
        GetItemRemoveItemError, ItemChangeAutochargeError, ItemChangeBoosterError, ItemChangeCharacterError,
        ItemChangeChargeError, ItemChangeDroneError, ItemChangeFighterError, ItemChangeFwEffectError,
        ItemChangeImplantError, ItemChangeModuleError, ItemChangeProjEffectError, ItemChangeRigError,
        ItemChangeServiceError, ItemChangeShipError, ItemChangeSkillError, ItemChangeStanceError, ItemRemoveItemError,
    },
    fit::{ChangeFitError, GetFitError},
    fleet::{ChangeFleetError, GetFleetError},
    item::{GetItemError, RemoveItemError},
    sol::{AddSolError, ChangeSolError, GetSolError, RemoveSolError},
    src::{AddSrcError, GetSrcError, RemoveSrcError},
};

pub mod core {
    pub use rc::err::*;
}
