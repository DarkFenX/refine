#[cfg(feature = "serde")]
pub use rc::err::{ParseFitIdError, ParseFleetIdError, ParseItemIdError};

#[cfg(feature = "serde")]
pub use crate::api::ParseSolarSystemIdError;
pub use crate::{
    api::{
        CtlFitChangeError, CtlSolChangeError, FitGetError, FleetGetError, ItemGetError, SolAddError, SolGetError,
        SolRemoveError, SolSwitchSrcError,
    },
    ctl::{
        AutochargeChangeError, BackrefRenderError, BoosterChangeError, CharacterChangeError, ChargeChangeError,
        DroneAddError, DroneChangeError, FighterAddError, FighterChangeError, FitAddError, FitChangeError,
        FitCharacterChangeError, FitCtlCmdError, FitGetBoosterAddError, FitGetCharacterChangeError,
        FitGetCharacterSetError, FitGetCharacterUnsetError, FitGetDroneAddError, FitGetFighterAddError,
        FitGetFitChangeError, FitGetFitRemoveError, FitGetFwEffectAddError, FitGetImplantAddError,
        FitGetModuleAddError, FitGetRigAddError, FitGetServiceAddError, FitGetShipChangeError, FitGetShipSetError,
        FitGetShipUnsetError, FitGetSkillAddError, FitGetStanceChangeError, FitGetStanceSetError,
        FitGetStanceUnsetError, FitGetSubsystemAddError, FitShipChangeError, FitStanceChangeError, FleetAddError,
        FleetChangeError, FleetGetFleetChangeError, FleetGetFleetRemoveError, FleetRemoveCmd, FwEffectChangeError,
        ImplantChangeError, ItemAddError, ItemCharacterChangeError, ItemCtlError, ItemGetAutochargeChangeError,
        ItemGetBoosterChangeError, ItemGetCharacterChangeError, ItemGetChargeChangeError, ItemGetDroneChangeError,
        ItemGetFighterChangeError, ItemGetFwEffectChangeError, ItemGetImplantChangeError, ItemGetItemRemoveError,
        ItemGetModuleChangeError, ItemGetProjEffectChangeError, ItemGetRigChangeError, ItemGetServiceChangeError,
        ItemGetShipChangeError, ItemGetSkillChangeError, ItemGetStanceChangeError, ItemGetSubsystemChangeError,
        ItemGetSwEffectChangeError, ItemRemoveError, ItemShipChangeError, ItemStanceChangeError, ModuleAddError,
        ModuleChangeError, ProjEffectAddError, ProjEffectChangeError, RigChangeError, ServiceChangeError,
        ShipChangeError, SkillAddError, SkillChangeError, SolCtlError, StanceChangeError, SubsystemChangeError,
        SwEffectChangeError,
    },
};

pub mod core {
    pub use rc::err::*;
}
