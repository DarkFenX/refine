pub mod basic;
pub use crate::{
    sol::{
        api::{
            AddDroneMutationError, AddDroneProjError, AddFighterProjError, AddModuleMutationError, AddModuleProjError,
            AddProjError, AddSkillError, ChangeDroneMutationError, ChangeModuleMutationError, FleetAddFitError,
            FleetRemoveFitError, GetAutochargeError, GetBoosterError, GetCharacterError, GetChargeError, GetDroneError,
            GetFighterError, GetFitError, GetFleetError, GetFwEffectError, GetImplantError, GetItemAttrError,
            GetItemError, GetModuleError, GetProjEffectError, GetProjError, GetRangedProjError, GetRigError,
            GetServiceError, GetShipError, GetSkillError, GetStanceError, GetSubsystemError, GetSwEffectError,
            IterItemAttrsError, IterItemEffectsError, IterItemModifiersError, RemoveDroneMutationError,
            RemoveFitRahIncomingDpsError, RemoveItemError, RemoveModuleMutationError, SetFighterCountOverrideError,
            SetFitFleetError, SetFitSecStatusError, SetSkillLevelError, UnsetFitFleetError,
        },
        misc::{NewBreacherInfoError, NewDpsProfileError},
    },
    src::SrcInitError,
};
