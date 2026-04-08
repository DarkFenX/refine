pub mod basic;
pub use crate::{
    api::{
        AddMutationError, AddProjError, AddSkillError, AttrIdParseError, AttrMutateRawError, EffectIdParseError,
        FitAppliedStatError, FitCharacterStatError, FitShipAppliedStatError, FitShipStatError, FleetAddFitError,
        FleetRemoveFitError, FleetStatAppliedError, GetAbilityError, GetAutochargeError, GetBoosterError,
        GetCharacterError, GetChargeError, GetDroneError, GetFighterError, GetFitError, GetFleetError,
        GetFwEffectError, GetImplantError, GetItemAttrError, GetItemError, GetModuleError, GetProjEffectError,
        GetProjError, GetRangedProjError, GetRawMAttrError, GetRigError, GetServiceError, GetShipError, GetSkillError,
        GetStanceError, GetSubsystemError, GetSwEffectError, ItemAppliedStatError, ItemStatError, IterItemAttrsError,
        IterItemEffectsError, IterItemModifiersError, RemoveFitRahIncomingDpsError, RemoveItemError, SetFitFleetError,
        SetSkillTypeIdError, UnsetFitFleetError,
    },
    misc::{BreacherProfileError, DpsProfileError},
    num::{FighterCountError, FitSecStatusError, SkillLevelError, UnitIntervalError},
    rd::SrcInitError,
};
