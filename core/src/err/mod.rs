pub mod basic;
pub use crate::{
    misc::{
        BreacherInfoError, DpsProfileError, FighterCountOverrideError, FitSecStatusError, SkillLevelError,
        UnitIntervalError,
    },
    sol::api::{
        AddMutationError, AddProjError, AddRangedProjError, AddSkillError, AttrMutateRawError, FleetAddFitError,
        FleetRemoveFitError, GetAutochargeError, GetBoosterError, GetCharacterError, GetChargeError, GetDroneError,
        GetFighterError, GetFitError, GetFleetError, GetFwEffectError, GetImplantError, GetItemAttrError, GetItemError,
        GetModuleError, GetProjEffectError, GetProjError, GetRangedProjError, GetRawMAttrError, GetRigError,
        GetServiceError, GetShipError, GetSkillError, GetStanceError, GetSubsystemError, GetSwEffectError,
        IterItemAttrsError, IterItemEffectsError, IterItemModifiersError, RemoveFitRahIncomingDpsError,
        RemoveItemError, SetFitFleetError, SetSkillTypeIdError, UnsetFitFleetError,
    },
    src::SrcInitError,
};
