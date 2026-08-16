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
        FitGetFitRemoveError, FitGetImplantAddError, FitGetRigAddError, FitGetSubsystemAddError, FleetAddError,
        FleetChangeError, FleetGetFleetChangeError, FleetGetFleetRemoveError, FleetRemoveCmd, GetFitAddDroneError,
        GetFitAddFighterError, GetFitAddFwEffectError, GetFitAddModuleError, GetFitAddServiceError,
        GetFitAddSkillError, GetFitChangeCharacterError, GetFitChangeShipError, GetFitChangeStanceError,
        GetFitSetCharacterError, GetFitSetShipError, GetFitSetStanceError, GetFitUnsetCharacterError,
        GetFitUnsetShipError, GetFitUnsetStanceError, GetItemChangeCharacterError, GetItemChangeDroneError,
        GetItemChangeFighterError, GetItemChangeFwEffectError, GetItemChangeModuleError, GetItemChangeProjEffectError,
        GetItemChangeServiceError, GetItemChangeShipError, GetItemChangeSkillError, GetItemChangeStanceError,
        GetItemChangeSwEffectError, ImplantChangeError, ItemAddError, ItemChangeCharacterError, ItemChangeDroneError,
        ItemChangeFighterError, ItemChangeFwEffectError, ItemChangeModuleError, ItemChangeProjEffectError,
        ItemChangeServiceError, ItemChangeShipError, ItemChangeSkillError, ItemChangeStanceError,
        ItemChangeSwEffectError, ItemCtlError, ItemGetAutochargeChangeError, ItemGetBoosterChangeError,
        ItemGetChargeChangeError, ItemGetImplantChangeError, ItemGetItemRemoveError, ItemGetRigChangeError,
        ItemGetSubsystemChangeError, ItemRemoveError, RigChangeError, SubsystemChangeError,
    },
};

pub mod core {
    pub use rc::err::*;
}
