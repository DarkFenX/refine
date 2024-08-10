//! Solar system extension methods which handle item manipulation.

pub use misc::{SolOrdAddMode, SolOrdRmMode};
pub use sole_autocharge::GetAutochargeInfoError;
pub use sole_booster::{
    AddBoosterError, GetBoosterInfoError, GetFitBoosterInfosError, RemoveBoosterError, SetBoosterSideEffectStateError,
    SetBoosterStateError,
};
pub use sole_character::{
    GetFitCharacterInfoError, RemoveCharacterError, RemoveFitCharacterError, SetCharacterStateError,
    SetFitCharacterError, SetFitCharacterStateError,
};
pub use sole_charge::{GetChargeInfoError, RemoveChargeError};
pub use sole_drone::{AddDroneError, GetDroneInfoError, GetFitDroneInfosError, RemoveDroneError, SetDroneStateError};
pub use sole_fighter::{
    AddFighterError, GetFighterInfoError, GetFitFighterInfosError, RemoveFighterError, SetFighterStateError,
};
pub use sole_fw_effect::{
    AddFwEffectError, GetFitFwEffectInfosError, GetFwEffectInfoError, RemoveFwEffectError, SetFwEffectStateError,
};
pub use sole_implant::{
    AddImplantError, GetFitImplantInfosError, GetImplantInfoError, RemoveImplantError, SetImplantStateError,
};
pub use sole_item::{GetItemInfoError, RemoveItemError};
pub use sole_module::{
    AddModuleError, AddModuleProjError, AddSetModuleChargeError, ChangeModuleProjError, GetFitModuleInfosError,
    GetModuleInfoError, RemoveModuleChargeError, RemoveModuleError, RemoveModuleProjError, SetModuleStateError,
};
pub use sole_proj_effect::{
    AddProjEffectError, AddProjEffectProjError, GetProjEffectInfoError, RemoveProjEffectError,
    RemoveProjEffectProjError, SetProjEffectStateError,
};
pub use sole_rig::{AddRigError, GetFitRigInfosError, GetRigInfoError, RemoveRigError, SetRigStateError};
pub use sole_ship::{
    GetFitShipInfoError, RemoveFitShipError, RemoveShipError, SetFitShipError, SetFitShipStateError, SetShipStateError,
};
pub use sole_skill::{
    AddSkillError, GetFitSkillInfosError, GetSkillInfoError, RemoveSkillError, SetSkillLevelError, SetSkillStateError,
};
pub use sole_stance::{
    GetFitStanceInfoError, RemoveFitStanceError, RemoveStanceError, SetFitStanceError, SetFitStanceStateError,
    SetStanceStateError,
};
pub use sole_subsystem::{
    AddSubsystemError, GetFitSubsystemInfosError, GetSubsystemInfoError, RemoveSubsystemError, SetSubsystemStateError,
};
pub use sole_sw_effect::{AddSwEffectError, GetSwEffectInfoError, RemoveSwEffectError, SetSwEffectStateError};

mod misc;
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
mod sole_ship;
mod sole_skill;
mod sole_stance;
mod sole_subsystem;
mod sole_sw_effect;
