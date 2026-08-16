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
        FitAddFighterError, FitAddModuleError, FitChangeCharacterError, FitChangeError, FitChangeShipError,
        FitChangeStanceError, FitCtlCmdError, FitGetBoosterAddError, FitGetFitChangeError, FitGetFitRemoveError,
        FitGetFwEffectAddError, FitGetImplantAddError, FitGetRigAddError, FitGetServiceAddError, FitGetSkillAddError,
        FitGetSubsystemAddError, FleetAddError, FleetChangeError, FleetGetFleetChangeError, FleetGetFleetRemoveError,
        FleetRemoveCmd, FwEffectChangeError, GetFitAddDroneError, GetFitAddFighterError, GetFitAddModuleError,
        GetFitChangeCharacterError, GetFitChangeShipError, GetFitChangeStanceError, GetFitSetCharacterError,
        GetFitSetShipError, GetFitSetStanceError, GetFitUnsetCharacterError, GetFitUnsetShipError,
        GetFitUnsetStanceError, GetItemChangeCharacterError, GetItemChangeDroneError, GetItemChangeFighterError,
        GetItemChangeModuleError, GetItemChangeProjEffectError, GetItemChangeShipError, GetItemChangeStanceError,
        GetItemChangeSwEffectError, ImplantChangeError, ItemAddError, ItemChangeCharacterError, ItemChangeDroneError,
        ItemChangeFighterError, ItemChangeModuleError, ItemChangeProjEffectError, ItemChangeShipError,
        ItemChangeStanceError, ItemChangeSwEffectError, ItemCtlError, ItemGetAutochargeChangeError,
        ItemGetBoosterChangeError, ItemGetChargeChangeError, ItemGetFwEffectChangeError, ItemGetImplantChangeError,
        ItemGetItemRemoveError, ItemGetRigChangeError, ItemGetServiceChangeError, ItemGetSkillChangeError,
        ItemGetSubsystemChangeError, ItemRemoveError, RigChangeError, ServiceChangeError, SkillAddError,
        SkillChangeError, SubsystemChangeError,
    },
};

pub mod core {
    pub use rc::err::*;
}
