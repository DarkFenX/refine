pub mod basic;
pub use crate::{
    sol::{
        sole_calc::{
            GetItemAttrError, IterItemAttrsError, IterItemEffectsError, IterItemModifiersError, SetItemEffectModeError,
            SetItemEffectModesError,
        },
        sole_fit::{GetFitError, RemoveFitError, SetFitFleetError, UnsetFitFleetError},
        sole_fleet::{GetFleetError, RemoveFleetError},
        sole_item::{
            AddBoosterError, AddDroneError, AddDroneProjError, AddFighterError, AddFighterProjError, AddFwEffectError,
            AddImplantError, AddModuleError, AddModuleProjError, AddProjEffectProjError, AddRigError, AddSkillError,
            AddSubsystemError, ChangeDroneProjError, ChangeFighterProjError, ChangeModuleProjError, GetAutochargeError,
            GetBoosterError, GetChargeError, GetDroneError, GetFighterError, GetFitBoostersError, GetFitCharacterError,
            GetFitDronesError, GetFitFightersError, GetFitFwEffectsError, GetFitImplantsError, GetFitModulesError,
            GetFitRigsError, GetFitShipError, GetFitSkillsError, GetFitStanceError, GetFitSubsystemsError,
            GetFwEffectError, GetImplantError, GetItemError, GetModuleError, GetProjEffectError, GetRigError,
            GetSkillError, GetSubsystemError, GetSwEffectError, RemoveBoosterError, RemoveCharacterError,
            RemoveChargeError, RemoveDroneError, RemoveDroneProjError, RemoveFighterError, RemoveFighterProjError,
            RemoveFitCharacterError, RemoveFitShipError, RemoveFitStanceError, RemoveFwEffectError, RemoveImplantError,
            RemoveItemError, RemoveModuleChargeError, RemoveModuleError, RemoveModuleProjError, RemoveProjEffectError,
            RemoveProjEffectProjError, RemoveRigError, RemoveShipError, RemoveSkillError, RemoveStanceError,
            RemoveSubsystemError, RemoveSwEffectError, SetAutochargeStateError, SetBoosterSideEffectStateError,
            SetBoosterStateError, SetCharacterStateError, SetChargeStateError, SetDroneStateError,
            SetFighterStateError, SetFitCharacterError, SetFitCharacterStateError, SetFitShipError,
            SetFitShipStateError, SetFitStanceError, SetFitStanceStateError, SetFwEffectStateError,
            SetImplantStateError, SetModuleChargeError, SetModuleStateError, SetProjEffectStateError, SetRigStateError,
            SetShipStateError, SetSkillLevelError, SetSkillStateError, SetStanceStateError, SetSubsystemStateError,
            SetSwEffectStateError,
        },
    },
    src::SrcInitError,
};
