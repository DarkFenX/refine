use crate::{
    CtlCmdResp, CtlCmdResps, FitAddBoosterCmd, FitAddDroneCmd, FitAddFighterCmd, FitAddFwEffectCmd, FitAddImplantCmd,
    FitAddModuleCmd, FitAddRigCmd, FitAddServiceCmd, FitAddSkillCmd, FitAddSubsystemCmd, FitChangeAutochargeCmd,
    FitChangeBoosterCmd, FitChangeCharacterCmd, FitChangeChargeCmd, FitChangeDroneCmd, FitChangeFighterCmd,
    FitChangeFitCmd, FitChangeFwEffectCmd, FitChangeImplantCmd, FitChangeModuleCmd, FitChangeRigCmd,
    FitChangeServiceCmd, FitChangeShipCmd, FitChangeSkillCmd, FitChangeStanceCmd, FitChangeSubsystemCmd,
    FitRemoveItemCmd, FitSetCharacterCmd, FitSetShipCmd, FitSetStanceCmd, FitUnsetCharacterCmd, FitUnsetShipCmd,
    FitUnsetStanceCmd,
    ctl::inner::{
        ICmdAutochargeChangeFCtxRIds, ICmdBoosterAddICtx, ICmdBoosterChangeFCtxRIds, ICmdCharacterChangeICtx,
        ICmdCharacterSetICtx, ICmdCharacterUnsetICtx, ICmdChargeChangeFCtxRIds, ICmdDroneAddICtxRIds,
        ICmdDroneChangeFCtxRIds, ICmdFighterAddICtxRIds, ICmdFighterChangeFCtxRIds, ICmdFitChangeICtxRIds,
        ICmdFwEffectAddICtx, ICmdFwEffectChangeFCtxRIds, ICmdImplantAddICtx, ICmdImplantChangeFCtxRIds,
        ICmdItemRemoveFCtxRIds, ICmdModuleAddICtxRIds, ICmdModuleChangeFCtxRIds, ICmdRigAddICtx, ICmdRigChangeFCtxRIds,
        ICmdServiceAddICtx, ICmdServiceChangeFCtxRIds, ICmdShipChangeICtx, ICmdShipSetICtx, ICmdShipUnsetICtx,
        ICmdSkillAddICtx, ICmdSkillChangeFCtxRIds, ICmdStanceChangeICtx, ICmdStanceSetICtx, ICmdStanceUnsetICtx,
        ICmdSubsystemAddICtx, ICmdSubsystemChangeFCtxRIds,
    },
    err::{
        BackrefRenderError, FitAddDroneError, FitAddFighterError, FitAddModuleError, FitAddSkillError,
        FitChangeCharacterError, FitChangeFitError, FitChangeShipError, FitChangeStanceError,
        GetItemChangeAutochargeError, GetItemChangeBoosterError, GetItemChangeChargeError, GetItemChangeDroneError,
        GetItemChangeFighterError, GetItemChangeFwEffectError, GetItemChangeImplantError, GetItemChangeModuleError,
        GetItemChangeRigError, GetItemChangeServiceError, GetItemChangeSkillError, GetItemChangeSubsystemError,
        GetItemRemoveItemError,
    },
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub enum ChangeFitEnumCmd {
    // Fit
    ChangeFit(FitChangeFitCmd),
    // Item
    RemoveItem(FitRemoveItemCmd),
    // Item - autocharge
    ChangeAutocharge(FitChangeAutochargeCmd),
    // Item - booster
    AddBooster(FitAddBoosterCmd),
    ChangeBooster(FitChangeBoosterCmd),
    // Item - character
    SetCharacter(FitSetCharacterCmd),
    ChangeCharacter(FitChangeCharacterCmd),
    UnsetCharacter(FitUnsetCharacterCmd),
    // Item - charge
    ChangeCharge(FitChangeChargeCmd),
    // Item - drone
    AddDrone(FitAddDroneCmd),
    ChangeDrone(FitChangeDroneCmd),
    // Item - fighter
    AddFighter(FitAddFighterCmd),
    ChangeFighter(FitChangeFighterCmd),
    // Item - fit-wide effect
    AddFwEffect(FitAddFwEffectCmd),
    ChangeFwEffect(FitChangeFwEffectCmd),
    // Item - implant
    AddImplant(FitAddImplantCmd),
    ChangeImplant(FitChangeImplantCmd),
    // Item - module
    AddModule(FitAddModuleCmd),
    ChangeModule(FitChangeModuleCmd),
    // Item - rig
    AddRig(FitAddRigCmd),
    ChangeRig(FitChangeRigCmd),
    // Item - service
    AddService(FitAddServiceCmd),
    ChangeService(FitChangeServiceCmd),
    // Item - ship
    SetShip(FitSetShipCmd),
    ChangeShip(FitChangeShipCmd),
    UnsetShip(FitUnsetShipCmd),
    // Item - skill
    AddSkill(FitAddSkillCmd),
    ChangeSkill(FitChangeSkillCmd),
    // Item - stance
    SetStance(FitSetStanceCmd),
    ChangeStance(FitChangeStanceCmd),
    UnsetStance(FitUnsetStanceCmd),
    // Item - subsystem
    AddSubsystem(FitAddSubsystemCmd),
    ChangeSubsystem(FitChangeSubsystemCmd),
}

pub(crate) enum ChangeFitEnumCmdRIds {
    // Fit
    ChangeFit(ICmdFitChangeICtxRIds),
    // Item
    RemoveItem(ICmdItemRemoveFCtxRIds),
    // Item - autocharge
    ChangeAutocharge(ICmdAutochargeChangeFCtxRIds),
    // Item - booster
    AddBooster(ICmdBoosterAddICtx),
    ChangeBooster(ICmdBoosterChangeFCtxRIds),
    // Item - character
    SetCharacter(ICmdCharacterSetICtx),
    ChangeCharacter(ICmdCharacterChangeICtx),
    UnsetCharacter(ICmdCharacterUnsetICtx),
    // Item - charge
    ChangeCharge(ICmdChargeChangeFCtxRIds),
    // Item - drone
    AddDrone(ICmdDroneAddICtxRIds),
    ChangeDrone(ICmdDroneChangeFCtxRIds),
    // Item - fighter
    AddFighter(ICmdFighterAddICtxRIds),
    ChangeFighter(ICmdFighterChangeFCtxRIds),
    // Item - fit-wide effect
    AddFwEffect(ICmdFwEffectAddICtx),
    ChangeFwEffect(ICmdFwEffectChangeFCtxRIds),
    // Item - implant
    AddImplant(ICmdImplantAddICtx),
    ChangeImplant(ICmdImplantChangeFCtxRIds),
    // Item - module
    AddModule(ICmdModuleAddICtxRIds),
    ChangeModule(ICmdModuleChangeFCtxRIds),
    // Item - rig
    AddRig(ICmdRigAddICtx),
    ChangeRig(ICmdRigChangeFCtxRIds),
    // Item - service
    AddService(ICmdServiceAddICtx),
    ChangeService(ICmdServiceChangeFCtxRIds),
    // Item - ship
    SetShip(ICmdShipSetICtx),
    ChangeShip(ICmdShipChangeICtx),
    UnsetShip(ICmdShipUnsetICtx),
    // Item - skill
    AddSkill(ICmdSkillAddICtx),
    ChangeSkill(ICmdSkillChangeFCtxRIds),
    // Item - stance
    SetStance(ICmdStanceSetICtx),
    ChangeStance(ICmdStanceChangeICtx),
    UnsetStance(ICmdStanceUnsetICtx),
    // Item - subsystem
    AddSubsystem(ICmdSubsystemAddICtx),
    ChangeSubsystem(ICmdSubsystemChangeFCtxRIds),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeFitEnumCmd {
    pub(crate) fn render(self, resps: &CtlCmdResps) -> Result<ChangeFitEnumCmdRIds, BackrefRenderError> {
        Ok(match self {
            // Fit
            Self::ChangeFit(cmd) => ChangeFitEnumCmdRIds::ChangeFit(cmd.inner.render(resps)?),
            // Item
            Self::RemoveItem(cmd) => ChangeFitEnumCmdRIds::RemoveItem(cmd.inner.render(resps)?),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => ChangeFitEnumCmdRIds::ChangeAutocharge(cmd.inner.render(resps)?),
            // Item - booster
            Self::AddBooster(cmd) => ChangeFitEnumCmdRIds::AddBooster(cmd.inner),
            Self::ChangeBooster(cmd) => ChangeFitEnumCmdRIds::ChangeBooster(cmd.inner.render(resps)?),
            // Item - character
            Self::SetCharacter(cmd) => ChangeFitEnumCmdRIds::SetCharacter(cmd.inner),
            Self::ChangeCharacter(cmd) => ChangeFitEnumCmdRIds::ChangeCharacter(cmd.inner),
            Self::UnsetCharacter(cmd) => ChangeFitEnumCmdRIds::UnsetCharacter(cmd.inner),
            // Item - charge
            Self::ChangeCharge(cmd) => ChangeFitEnumCmdRIds::ChangeCharge(cmd.inner.render(resps)?),
            // Item - drone
            Self::AddDrone(cmd) => ChangeFitEnumCmdRIds::AddDrone(cmd.inner.render(resps)?),
            Self::ChangeDrone(cmd) => ChangeFitEnumCmdRIds::ChangeDrone(cmd.inner.render(resps)?),
            // Item - fighter
            Self::AddFighter(cmd) => ChangeFitEnumCmdRIds::AddFighter(cmd.inner.render(resps)?),
            Self::ChangeFighter(cmd) => ChangeFitEnumCmdRIds::ChangeFighter(cmd.inner.render(resps)?),
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => ChangeFitEnumCmdRIds::AddFwEffect(cmd.inner),
            Self::ChangeFwEffect(cmd) => ChangeFitEnumCmdRIds::ChangeFwEffect(cmd.inner.render(resps)?),
            // Item - implant
            Self::AddImplant(cmd) => ChangeFitEnumCmdRIds::AddImplant(cmd.inner),
            Self::ChangeImplant(cmd) => ChangeFitEnumCmdRIds::ChangeImplant(cmd.inner.render(resps)?),
            // Item - drone
            Self::AddModule(cmd) => ChangeFitEnumCmdRIds::AddModule(cmd.inner.render(resps)?),
            Self::ChangeModule(cmd) => ChangeFitEnumCmdRIds::ChangeModule(cmd.inner.render(resps)?),
            // Item - rig
            Self::AddRig(cmd) => ChangeFitEnumCmdRIds::AddRig(cmd.inner),
            Self::ChangeRig(cmd) => ChangeFitEnumCmdRIds::ChangeRig(cmd.inner.render(resps)?),
            // Item - service
            Self::AddService(cmd) => ChangeFitEnumCmdRIds::AddService(cmd.inner),
            Self::ChangeService(cmd) => ChangeFitEnumCmdRIds::ChangeService(cmd.inner.render(resps)?),
            // Item - ship
            Self::SetShip(cmd) => ChangeFitEnumCmdRIds::SetShip(cmd.inner),
            Self::ChangeShip(cmd) => ChangeFitEnumCmdRIds::ChangeShip(cmd.inner),
            Self::UnsetShip(cmd) => ChangeFitEnumCmdRIds::UnsetShip(cmd.inner),
            // Item - skill
            Self::AddSkill(cmd) => ChangeFitEnumCmdRIds::AddSkill(cmd.inner),
            Self::ChangeSkill(cmd) => ChangeFitEnumCmdRIds::ChangeSkill(cmd.inner.render(resps)?),
            // Item - stance
            Self::SetStance(cmd) => ChangeFitEnumCmdRIds::SetStance(cmd.inner),
            Self::ChangeStance(cmd) => ChangeFitEnumCmdRIds::ChangeStance(cmd.inner),
            Self::UnsetStance(cmd) => ChangeFitEnumCmdRIds::UnsetStance(cmd.inner),
            // Item - subsystem
            Self::AddSubsystem(cmd) => ChangeFitEnumCmdRIds::AddSubsystem(cmd.inner),
            Self::ChangeSubsystem(cmd) => ChangeFitEnumCmdRIds::ChangeSubsystem(cmd.inner.render(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeFitEnumCmdRIds {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> Result<CtlCmdResp, ChangeFitEnumError> {
        match self {
            // Fit
            Self::ChangeFit(cmd) => Ok(cmd.execute(core_fit)?.into()),
            // Item
            Self::RemoveItem(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - booster
            Self::AddBooster(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeBooster(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - character
            Self::SetCharacter(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeCharacter(cmd) => Ok(cmd.execute_via_fit(core_fit)?.into()),
            #[expect(clippy::unit_arg)]
            Self::UnsetCharacter(cmd) => Ok(cmd.execute(core_fit).into()),
            // Item - charge
            Self::ChangeCharge(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - drone
            Self::AddDrone(cmd) => Ok(cmd.execute(core_fit)?.into()),
            Self::ChangeDrone(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - fighter
            Self::AddFighter(cmd) => Ok(cmd.execute(core_fit)?.into()),
            Self::ChangeFighter(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeFwEffect(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - implant
            Self::AddImplant(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeImplant(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - module
            Self::AddModule(cmd) => Ok(cmd.execute(core_fit)?.into()),
            Self::ChangeModule(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - rig
            Self::AddRig(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeRig(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - service
            Self::AddService(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeService(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - ship
            Self::SetShip(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeShip(cmd) => Ok(cmd.execute_via_fit(core_fit)?.into()),
            #[expect(clippy::unit_arg)]
            Self::UnsetShip(cmd) => Ok(cmd.execute(core_fit).into()),
            // Item - skill
            Self::AddSkill(cmd) => Ok(cmd.execute(core_fit)?.into()),
            Self::ChangeSkill(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - stance
            Self::SetStance(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeStance(cmd) => Ok(cmd.execute_via_fit(core_fit)?.into()),
            #[expect(clippy::unit_arg)]
            Self::UnsetStance(cmd) => Ok(cmd.execute(core_fit).into()),
            // Item - subsystem
            Self::AddSubsystem(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeSubsystem(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeFitEnumError {
    // Fit
    #[error("failed to change fleet")]
    FitChange(#[from] FitChangeFitError),
    // Item
    #[error("failed to remove item")]
    ItemRemove(#[from] GetItemRemoveItemError),
    // Item - autocharge
    #[error("failed to change autocharge")]
    AutochargeChange(#[from] GetItemChangeAutochargeError),
    // Item - booster
    #[error("failed to change booster")]
    BoosterChange(#[from] GetItemChangeBoosterError),
    // Item - character
    #[error("failed to change character")]
    CharacterChange(#[from] FitChangeCharacterError),
    // Item - charge
    #[error("failed to change charge")]
    ChargeChange(#[from] GetItemChangeChargeError),
    // Item - drone
    #[error("failed to add drone")]
    DroneAdd(#[from] FitAddDroneError),
    #[error("failed to change drone")]
    DroneChange(#[from] GetItemChangeDroneError),
    // Item - fighter
    #[error("failed to add fighter")]
    FighterAdd(#[from] FitAddFighterError),
    #[error("failed to change fighter")]
    FighterChange(#[from] GetItemChangeFighterError),
    // Item - fit-wide effect
    #[error("failed to change fit-wide effect")]
    FwEffectChange(#[from] GetItemChangeFwEffectError),
    // Item - implant
    #[error("failed to change implant")]
    ImplantChange(#[from] GetItemChangeImplantError),
    // Item - module
    #[error("failed to add module")]
    ModuleAdd(#[from] FitAddModuleError),
    #[error("failed to change module")]
    ModuleChange(#[from] GetItemChangeModuleError),
    // Item - rig
    #[error("failed to change rig")]
    RigChange(#[from] GetItemChangeRigError),
    // Item - service
    #[error("failed to change service")]
    ServiceChange(#[from] GetItemChangeServiceError),
    // Item - ship
    #[error("failed to change ship")]
    ShipChange(#[from] FitChangeShipError),
    // Item - skill
    #[error("failed to add skill")]
    SkillAdd(#[from] FitAddSkillError),
    #[error("failed to change skill")]
    SkillChange(#[from] GetItemChangeSkillError),
    // Item - stance
    #[error("failed to change stance")]
    StanceChange(#[from] FitChangeStanceError),
    // Item - subsystem
    #[error("failed to change subsystem")]
    SubsystemChange(#[from] GetItemChangeSubsystemError),
}
