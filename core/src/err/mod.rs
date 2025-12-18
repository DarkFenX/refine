pub mod basic;
pub use crate::{
    api::{
        AddMutationError, AddProjError, AddSkillError, AttrMutateRawError, FitCharacterStatError, FitSecStatusError,
        FitShipStatError, FitStatAppliedError, FleetAddFitError, FleetRemoveFitError, FleetStatAppliedError,
        GetAbilityError, GetAutochargeError, GetBoosterError, GetCharacterError, GetChargeError, GetDroneError,
        GetFighterError, GetFitError, GetFleetError, GetFwEffectError, GetImplantError, GetItemAttrError, GetItemError,
        GetModuleError, GetProjEffectError, GetProjError, GetRangedProjError, GetRawMAttrError, GetRigError,
        GetServiceError, GetShipError, GetSkillError, GetStanceError, GetSubsystemError, GetSwEffectError,
        ItemStatAppliedError, ItemStatError, IterItemAttrsError, IterItemEffectsError, IterItemModifiersError,
        RemoveFitRahIncomingDpsError, RemoveItemError, SetFitFleetError, SetSkillTypeIdError, UnsetFitFleetError,
    },
    misc::{BreacherInfoError, DpsProfileError, FighterCountOverrideError, SkillLevelError},
    rd::SrcInitError,
    util::UnitIntervalError,
};
