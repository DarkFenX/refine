pub mod basic;
pub use crate::{
    sol::{
        api::{
            AddDroneMutationError, AddModuleMutationError, AddProjError, AddRangedProjError, AddSkillError,
            ChangeDroneMutationError, ChangeModuleMutationError, FleetAddFitError, FleetRemoveFitError,
            GetAutochargeError, GetBoosterError, GetCharacterError, GetChargeError, GetDroneError, GetFighterError,
            GetFitError, GetFleetError, GetFwEffectError, GetImplantError, GetItemAttrError, GetItemError,
            GetModuleError, GetProjEffectError, GetProjError, GetRangedProjError, GetRigError, GetServiceError,
            GetShipError, GetSkillError, GetStanceError, GetSubsystemError, GetSwEffectError, IterItemAttrsError,
            IterItemEffectsError, IterItemModifiersError, RemoveDroneMutationError, RemoveFitRahIncomingDpsError,
            RemoveItemError, RemoveModuleMutationError, SetFighterCountOverrideError, SetFitFleetError,
            UnsetFitFleetError,
        },
        misc::{BreacherInfoError, DpsProfileError, FitSecStatusError, SkillLevelError, UnitIntervalError},
    },
    src::SrcInitError,
};
