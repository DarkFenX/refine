pub mod basic;
pub use crate::{
    sol::{
        sole_calc::{
            GetItemAttrError, IterItemAttrsError, IterItemEffectsError, IterItemModifiersError, SetItemEffectModeError,
            SetItemEffectModesError,
        },
        sole_fit::{AddFitError, GetFitError, RemoveFitError, SetFitFleetError, UnsetFitFleetError},
        sole_fleet::{AddFleetError, GetFleetError, RemoveFleetError},
        sole_item::{
            AddBoosterError, AddDroneError, AddFighterError, AddFighterProjError, AddFwEffectError, AddImplantError,
            AddModuleError, AddModuleProjError, AddProjEffectError, AddProjEffectProjError, AddRigError, AddSkillError,
            AddSubsystemError, AddSwEffectError, ChangeFighterProjError, ChangeModuleProjError, GetAutochargeError,
            GetBoosterError, GetChargeError, GetDroneError, GetFighterError, GetFitBoostersError, GetFitCharacterError,
            GetFitDronesError, GetFitFightersError, GetFitFwEffectsError, GetFitImplantsError, GetFitModulesError,
            GetFitRigsError, GetFitShipError, GetFitSkillsError, GetFitStanceError, GetFitSubsystemsError,
            GetFwEffectError, GetImplantError, GetItemError, GetModuleError, GetProjEffectError, GetRigError,
            GetSkillError, GetSubsystemError, GetSwEffectError, RemoveBoosterError, RemoveCharacterError,
            RemoveChargeError, RemoveDroneError, RemoveFighterError, RemoveFighterProjError, RemoveFitCharacterError,
            RemoveFitShipError, RemoveFitStanceError, RemoveFwEffectError, RemoveImplantError, RemoveItemError,
            RemoveModuleChargeError, RemoveModuleError, RemoveModuleProjError, RemoveProjEffectError,
            RemoveProjEffectProjError, RemoveRigError, RemoveShipError, RemoveSkillError, RemoveStanceError,
            RemoveSubsystemError, RemoveSwEffectError, SetAutochargeStateError, SetBoosterSideEffectStateError,
            SetBoosterStateError, SetCharacterStateError, SetChargeStateError, SetDroneStateError,
            SetFighterStateError, SetFitCharacterError, SetFitCharacterStateError, SetFitShipError,
            SetFitShipStateError, SetFitStanceError, SetFitStanceStateError, SetFwEffectStateError,
            SetImplantStateError, SetModuleChargeError, SetModuleStateError, SetProjEffectStateError, SetRigStateError,
            SetShipStateError, SetSkillLevelError, SetSkillStateError, SetStanceStateError, SetSubsystemStateError,
            SetSwEffectStateError,
        },
        sole_src::SetSrcError,
    },
    src::SrcInitError,
};
