//! Solar system extension methods which handle item manipulation.

pub use sole_autocharge::{GetAutochargeInfoError, SetAutochargeStateError};
pub use sole_booster::{
    AddBoosterError, GetBoosterInfoError, GetFitBoosterInfosError, RemoveBoosterError, SetBoosterSideEffectStateError,
    SetBoosterStateError,
};
pub use sole_character::{
    GetFitCharacterInfoError, RemoveCharacterError, RemoveFitCharacterError, SetCharacterStateError,
    SetFitCharacterError, SetFitCharacterStateError,
};
pub use sole_charge::{GetChargeInfoError, RemoveChargeError, SetChargeStateError};
pub use sole_drone::{
    AddDroneError, AddDroneMutationError, AddDroneProjError, ChangeDroneMutationError, ChangeDroneProjError,
    GetDroneInfoError, GetFitDroneInfosError, RemoveDroneError, RemoveDroneMutationError, RemoveDroneProjError,
    SetDroneStateError,
};
pub use sole_fighter::{
    AddFighterError, AddFighterProjError, ChangeFighterProjError, GetFighterInfoError, GetFitFighterInfosError,
    RemoveFighterCountOverrideError, RemoveFighterError, RemoveFighterProjError, SetFighterCountOverrideError,
    SetFighterStateError,
};
pub use sole_fw_effect::{
    AddFwEffectError, GetFitFwEffectInfosError, GetFwEffectInfoError, RemoveFwEffectError, SetFwEffectStateError,
};
pub use sole_implant::{
    AddImplantError, GetFitImplantInfosError, GetImplantInfoError, RemoveImplantError, SetImplantStateError,
};
pub use sole_item::{GetItemInfoError, RemoveItemError};
pub use sole_module::{
    AddMode, AddModuleError, AddModuleMutationError, AddModuleProjError, ChangeModuleMutationError,
    ChangeModuleProjError, GetFitModuleInfosError, GetModuleInfoError, RemoveModuleChargeError, RemoveModuleError,
    RemoveModuleMutationError, RemoveModuleProjError, RmMode, SetModuleChargeError, SetModuleStateError,
};
pub use sole_proj_effect::{
    AddProjEffectProjError, GetProjEffectInfoError, RemoveProjEffectError, RemoveProjEffectProjError,
    SetProjEffectStateError,
};
pub use sole_rig::{AddRigError, GetFitRigsError, GetRigInfoError, RemoveRigError, SetRigStateError};
pub use sole_service::{
    AddServiceError, GetFitServiceInfosError, GetServiceInfoError, RemoveServiceError, SetServiceStateError,
};
pub use sole_ship::{
    GetFitShipInfoError, RemoveFitShipError, RemoveShipError, SetFitShipError, SetFitShipStateError, SetShipStateError,
};
pub use sole_skill::{
    AddSkillError, GetFitSkillsError, GetSkillInfoError, RemoveSkillError, SetSkillLevelError, SetSkillStateError,
};
pub use sole_stance::{
    GetFitStanceInfoError, RemoveFitStanceError, RemoveStanceError, SetFitStanceError, SetFitStanceStateError,
    SetStanceStateError,
};
pub use sole_subsystem::{
    AddSubsystemError, GetFitSubsystemsError, GetSubsystemInfoError, RemoveSubsystemError, SetSubsystemStateError,
};
pub use sole_sw_effect::{GetSwEffectInfoError, RemoveSwEffectError, SetSwEffectStateError};

mod sole_autocharge;
mod sole_booster;
mod sole_character;
mod sole_charge;
mod sole_drone;
mod sole_fighter;
mod sole_fw_effect;
mod sole_implant;
mod sole_item;
mod sole_module;
mod sole_proj_effect;
mod sole_rig;
mod sole_service;
mod sole_ship;
mod sole_skill;
mod sole_stance;
mod sole_subsystem;
mod sole_sw_effect;
