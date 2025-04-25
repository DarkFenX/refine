pub mod basic;
pub use crate::{
    sol::{
        api::{
            AddDroneMutationError, AddDroneProjError, AddFighterProjError, AddModuleMutationError, AddModuleProjError,
            AddProjError, AddSkillError, ChangeDroneMutationError, ChangeDroneProjError, ChangeFighterProjError,
            ChangeModuleMutationError, ChangeModuleProjError, FleetAddFitError, FleetRemoveFitError,
            GetAutochargeError, GetBoosterError, GetCharacterError, GetChargeError, GetDroneError, GetFighterError,
            GetFitError, GetFleetError, GetFwEffectError, GetImplantError, GetItemAttrError, GetItemError,
            GetModuleError, GetProjEffectError, GetProjError, GetRigError, GetServiceError, GetShipError,
            GetSkillError, GetStanceError, GetSubsystemError, GetSwEffectError, IterItemAttrsError,
            IterItemEffectsError, IterItemModifiersError, RemoveDroneMutationError, RemoveDroneProjError,
            RemoveFighterProjError, RemoveFitRahIncomingDpsError, RemoveItemError, RemoveModuleMutationError,
            RemoveModuleProjError, SetFighterCountOverrideError, SetFitFleetError, SetFitSecStatusError,
            SetSkillLevelError, UnsetFitFleetError,
        },
        misc::{NewBreacherInfoError, NewDpsProfileError},
    },
    src::SrcInitError,
};
