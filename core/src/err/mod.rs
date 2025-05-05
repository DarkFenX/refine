pub mod basic;
pub use crate::{
    sol::{
        api::{
            AddMutationError, AddProjError, AddRangedProjError, AddSkillError, AttrMutateRawError, FleetAddFitError,
            FleetRemoveFitError, GetAutochargeError, GetBoosterError, GetCharacterError, GetChargeError, GetDroneError,
            GetFighterError, GetFitError, GetFleetError, GetFwEffectError, GetImplantError, GetItemAttrError,
            GetItemError, GetModuleError, GetProjEffectError, GetProjError, GetRangedProjError, GetRawMAttrError,
            GetRigError, GetServiceError, GetShipError, GetSkillError, GetStanceError, GetSubsystemError,
            GetSwEffectError, IterItemAttrsError, IterItemEffectsError, IterItemModifiersError,
            RemoveFitRahIncomingDpsError, RemoveItemError, SetFighterCountOverrideError, SetFitFleetError,
            UnsetFitFleetError,
        },
        misc::{BreacherInfoError, DpsProfileError, FitSecStatusError, SkillLevelError, UnitIntervalError},
    },
    src::SrcInitError,
};
