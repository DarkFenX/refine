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
        FitGetFitRemoveError, FitGetImplantAddError, FitGetRigAddError, FitGetServiceAddError, FitGetSubsystemAddError,
        FleetAddError, FleetChangeError, FleetGetFleetChangeError, FleetGetFleetRemoveError, FleetRemoveCmd,
        GetFitAddDroneError, GetFitAddFighterError, GetFitAddFwEffectError, GetFitAddModuleError, GetFitAddSkillError,
        GetFitChangeCharacterError, GetFitChangeShipError, GetFitChangeStanceError, GetFitSetCharacterError,
        GetFitSetShipError, GetFitSetStanceError, GetFitUnsetCharacterError, GetFitUnsetShipError,
        GetFitUnsetStanceError, GetItemChangeCharacterError, GetItemChangeDroneError, GetItemChangeFighterError,
        GetItemChangeFwEffectError, GetItemChangeModuleError, GetItemChangeProjEffectError, GetItemChangeShipError,
        GetItemChangeSkillError, GetItemChangeStanceError, GetItemChangeSwEffectError, ImplantChangeError,
        ItemAddError, ItemChangeCharacterError, ItemChangeDroneError, ItemChangeFighterError, ItemChangeFwEffectError,
        ItemChangeModuleError, ItemChangeProjEffectError, ItemChangeShipError, ItemChangeSkillError,
        ItemChangeStanceError, ItemChangeSwEffectError, ItemCtlError, ItemGetAutochargeChangeError,
        ItemGetBoosterChangeError, ItemGetChargeChangeError, ItemGetImplantChangeError, ItemGetItemRemoveError,
        ItemGetRigChangeError, ItemGetServiceChangeError, ItemGetSubsystemChangeError, ItemRemoveError, RigChangeError,
        ServiceChangeError, SubsystemChangeError,
    },
};

pub mod core {
    pub use rc::err::*;
}
