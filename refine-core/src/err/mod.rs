pub use basic::ItemKindMatchError;

pub use crate::{
    api::{
        AddMutationError, AddProjError, AttrIdParseError, AttrMutateRawError, CreateSkillError, EffectIdParseError,
        FitAppliedStatError, FitCharacterStatError, FitShipAppliedStatError, FitShipStatError, FleetAddFitError,
        FleetRemoveFitError, FleetStatAppliedError, GetAbilityError, GetAutochargeError, GetBoosterError,
        GetCharacterError, GetChargeError, GetDroneError, GetFighterError, GetFitError, GetFleetError,
        GetFwEffectError, GetImplantError, GetItemAttrError, GetItemError, GetModuleError, GetProjEffectError,
        GetProjError, GetRangedProjError, GetRawMAttrError, GetRigError, GetServiceError, GetShipError,
        GetSideEffectError, GetSkillError, GetStanceError, GetSubsystemError, GetSwEffectError, ItemAppliedStatError,
        ItemStatError, IterItemAttrsError, IterItemEffectsError, IterItemModifiersError, RemoveFitRahIncomingDpsError,
        RemoveItemError, SetFitFleetError, SetSkillTypeIdError, UnsetFitFleetError,
    },
    misc::{BreacherProfileError, DpsProfileError},
    num::{FighterCountError, FitSecStatusError, SkillLevelError, UnitIntervalError},
};

pub mod basic;
