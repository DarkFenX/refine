pub mod basic;
pub use crate::{
    misc::{BreacherInfoError, DpsProfileError, FighterCountOverrideError, FitSecStatusError, SkillLevelError},
    sol::api::{
        AddMutationError, AddProjError, AddSkillError, AttrMutateRawError, FitShipStatError, FleetAddFitError,
        FleetRemoveFitError, GetAbilityError, GetAutochargeError, GetBoosterError, GetCharacterError, GetChargeError,
        GetDroneError, GetFighterError, GetFitError, GetFleetError, GetFwEffectError, GetImplantError,
        GetItemAttrError, GetItemError, GetModuleError, GetProjEffectError, GetProjError, GetRangedProjError,
        GetRawMAttrError, GetRigError, GetServiceError, GetShipError, GetSkillError, GetStanceError, GetSubsystemError,
        GetSwEffectError, ItemStatError, IterItemAttrsError, IterItemEffectsError, IterItemModifiersError,
        RemoveFitRahIncomingDpsError, RemoveItemError, SetFitFleetError, SetSkillTypeIdError, UnsetFitFleetError,
    },
    src::SrcInitError,
    util::UnitIntervalError,
};
