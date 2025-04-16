pub mod basic;
pub use crate::{
    sol::{
        api::{
            AddDroneMutationError, AddDroneProjError, AddFighterProjError, AddModuleMutationError, AddModuleProjError,
            AddProjEffectProjError, AddSkillError, ChangeDroneMutationError, ChangeDroneProjError,
            ChangeFighterProjError, ChangeModuleMutationError, ChangeModuleProjError, GetAutochargeError,
            GetBoosterError, GetCharacterError, GetChargeError, GetDroneError, GetFighterError, GetFitInfoError,
            GetFleetInfoError, GetFwEffectError, GetImplantError, GetItemAttrError, GetItemError, GetModuleError,
            GetProjEffectError, GetRigError, GetServiceError, GetShipError, GetSkillError, GetStanceError,
            GetSubsystemError, GetSwEffectError, IterItemAttrsError, IterItemEffectsError, IterItemModifiersError,
            RemoveDroneMutationError, RemoveDroneProjError, RemoveFighterProjError, RemoveFitRahIncomingDpsError,
            RemoveItemError, RemoveModuleChargeError, RemoveModuleMutationError, RemoveModuleProjError,
            RemoveProjEffectProjError, SetFighterCountOverrideError, SetFitFleetError, SetFitSecStatusError,
            SetSkillLevelError, UnsetFitFleetError,
        },
        misc::{NewBreacherInfoError, NewDpsProfileError},
    },
    src::SrcInitError,
};
