pub use basic::ItemKindMatchError;

pub use crate::{
    api::{
        AddMutationError, AddProjError, AddSkillError, AttrMutateRawError, FitAppliedStatError, FitCharacterStatError,
        FitShipAppliedStatError, FitShipStatError, FleetAddFitError, FleetRemoveFitError, FleetStatAppliedError,
        GetAbilityError, GetAutochargeError, GetBoosterError, GetCharacterError, GetChargeError, GetDroneError,
        GetFighterError, GetFitError, GetFleetError, GetFwEffectError, GetImplantError, GetItemAttrError, GetItemError,
        GetModuleError, GetProjEffectError, GetProjError, GetRangedProjError, GetRawMAttrError, GetRigError,
        GetServiceError, GetShipError, GetSideEffectError, GetSkillError, GetStanceError, GetSubsystemError,
        GetSwEffectError, ItemAppliedStatError, ItemStatError, IterItemAttrsError, IterItemEffectsError,
        IterItemModifiersError, RemoveFitRahIncomingDpsError, RemoveItemError, SetFitFleetError, SetSkillTypeIdError,
        UnsetFitFleetError,
    },
    num::{CountNzError, FitSecStatusError, PValueError, SkillLevelError, UnitIntervalError},
};

pub mod basic;
