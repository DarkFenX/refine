use super::{sub_item_character::SolChangeCharacterCmdRIds, sub_item_ship::SolChangeShipCmdRIds};
use crate::cmd::{
    AddFitError, AddFleetError, AddProjEffectError, ChangeCharacterError, ChangeShipError, GetFitAddBoosterError,
    GetFitAddDroneError, GetFitAddFighterError, GetFitAddFwEffectError, GetFitAddImplantError, GetFitAddModuleError,
    GetFitAddRigError, GetFitAddServiceError, GetFitAddSkillError, GetFitChangeFitError, GetFitRemoveFitError,
    GetFitSetCharacterError, GetFitSetShipError, GetFitUnsetCharacterError, GetFitUnsetShipError,
    GetFleetChangeFleetError, GetFleetRemoveFleetError, GetItemChangeAutochargeError, GetItemChangeBoosterError,
    GetItemChangeChargeError, GetItemChangeDroneError, GetItemChangeFighterError, GetItemChangeFwEffectError,
    GetItemChangeImplantError, GetItemChangeModuleError, GetItemChangeProjEffectError, GetItemChangeRigError,
    GetItemChangeServiceError, GetItemChangeSkillError, GetItemRemoveItemError, SolAddBoosterCmd, SolAddDroneCmd,
    SolAddFighterCmd, SolAddFitCmd, SolAddFleetCmd, SolAddFwEffectCmd, SolAddImplantCmd, SolAddModuleCmd,
    SolAddProjEffectCmd, SolAddRigCmd, SolAddServiceCmd, SolAddSkillCmd, SolChangeAutochargeCmd, SolChangeBoosterCmd,
    SolChangeCharacterCmd, SolChangeChargeCmd, SolChangeDroneCmd, SolChangeFighterCmd, SolChangeFitCmd,
    SolChangeFleetCmd, SolChangeFwEffectCmd, SolChangeImplantCmd, SolChangeModuleCmd, SolChangeProjEffectCmd,
    SolChangeRigCmd, SolChangeServiceCmd, SolChangeShipCmd, SolChangeSkillCmd, SolChangeSolCmd, SolRemoveFitCmd,
    SolRemoveFleetCmd, SolRemoveItemCmd, SolSetCharacterCmd, SolSetShipCmd, SolUnsetCharacterCmd, SolUnsetShipCmd,
    inner::{
        ICmdAutochargeChangeFCtxRIds, ICmdBoosterAddFCtxRIds, ICmdBoosterChangeFCtxRIds, ICmdCharacterSetFCtxRIds,
        ICmdCharacterUnsetFCtxRIds, ICmdChargeChangeFCtxRIds, ICmdDroneAddFCtxRIds, ICmdDroneChangeFCtxRIds,
        ICmdFighterAddFCtxRIds, ICmdFighterChangeFCtxRIds, ICmdFitAddFCtxRIds, ICmdFitChangeFCtxRIds,
        ICmdFitRemoveFCtxRIds, ICmdFleetAddFCtxRIds, ICmdFleetChangeFCtxRIds, ICmdFleetRemoveFCtxRIds,
        ICmdFwEffectAddFCtxRIds, ICmdFwEffectChangeFCtxRIds, ICmdImplantAddFCtxRIds, ICmdImplantChangeFCtxRIds,
        ICmdItemRemoveFCtxRIds, ICmdModuleAddFCtxRIds, ICmdModuleChangeFCtxRIds, ICmdProjEffectAddFCtxRIds,
        ICmdProjEffectChangeFCtxRIds, ICmdRigAddFCtxRIds, ICmdRigChangeFCtxRIds, ICmdServiceAddFCtxRIds,
        ICmdServiceChangeFCtxRIds, ICmdShipSetFCtxRIds, ICmdShipUnsetFCtxRIds, ICmdSkillAddFCtxRIds,
        ICmdSkillChangeFCtxRIds, ICmdSolChangeFCtx,
    },
    shared::{BackrefRenderError, CmdResp, CmdResps},
};

pub enum ChangeSolEnumCmd {
    // Solar system
    ChangeSol(SolChangeSolCmd),
    // Fleet
    AddFleet(SolAddFleetCmd),
    ChangeFleet(SolChangeFleetCmd),
    RemoveFleet(SolRemoveFleetCmd),
    // Fit
    AddFit(SolAddFitCmd),
    ChangeFit(SolChangeFitCmd),
    RemoveFit(SolRemoveFitCmd),
    // Item
    RemoveItem(SolRemoveItemCmd),
    // Item - autocharge
    ChangeAutocharge(SolChangeAutochargeCmd),
    // Item - booster
    AddBooster(SolAddBoosterCmd),
    ChangeBooster(SolChangeBoosterCmd),
    // Item - character
    SetCharacter(SolSetCharacterCmd),
    ChangeCharacter(SolChangeCharacterCmd),
    UnsetCharacter(SolUnsetCharacterCmd),
    // Item - charge
    ChangeCharge(SolChangeChargeCmd),
    // Item - drone
    AddDrone(SolAddDroneCmd),
    ChangeDrone(SolChangeDroneCmd),
    // Item - fighter
    AddFighter(SolAddFighterCmd),
    ChangeFighter(SolChangeFighterCmd),
    // Item - fit-wide effect
    AddFwEffect(SolAddFwEffectCmd),
    ChangeFwEffect(SolChangeFwEffectCmd),
    // Item - implant
    AddImplant(SolAddImplantCmd),
    ChangeImplant(SolChangeImplantCmd),
    // Item - module
    AddModule(SolAddModuleCmd),
    ChangeModule(SolChangeModuleCmd),
    // Item - projected effect
    AddProjEffect(SolAddProjEffectCmd),
    ChangeProjEffect(SolChangeProjEffectCmd),
    // Item - rig
    AddRig(SolAddRigCmd),
    ChangeRig(SolChangeRigCmd),
    // Item - service
    AddService(SolAddServiceCmd),
    ChangeService(SolChangeServiceCmd),
    // Item - ship
    SetShip(SolSetShipCmd),
    ChangeShip(SolChangeShipCmd),
    UnsetShip(SolUnsetShipCmd),
    // Item - skill
    AddSkill(SolAddSkillCmd),
    ChangeSkill(SolChangeSkillCmd),
}

pub(crate) enum ChangeSolEnumCmdRIds {
    // Solar system
    ChangeSol(ICmdSolChangeFCtx),
    // Fleet
    AddFleet(ICmdFleetAddFCtxRIds),
    ChangeFleet(ICmdFleetChangeFCtxRIds),
    RemoveFleet(ICmdFleetRemoveFCtxRIds),
    // Fit
    AddFit(ICmdFitAddFCtxRIds),
    ChangeFit(ICmdFitChangeFCtxRIds),
    RemoveFit(ICmdFitRemoveFCtxRIds),
    // Item
    RemoveItem(ICmdItemRemoveFCtxRIds),
    // Item - autocharge
    ChangeAutocharge(ICmdAutochargeChangeFCtxRIds),
    // Item - booster
    AddBooster(ICmdBoosterAddFCtxRIds),
    ChangeBooster(ICmdBoosterChangeFCtxRIds),
    // Item - character
    SetCharacter(ICmdCharacterSetFCtxRIds),
    ChangeCharacter(SolChangeCharacterCmdRIds),
    UnsetCharacter(ICmdCharacterUnsetFCtxRIds),
    // Item - charge
    ChangeCharge(ICmdChargeChangeFCtxRIds),
    // Item - drone
    AddDrone(ICmdDroneAddFCtxRIds),
    ChangeDrone(ICmdDroneChangeFCtxRIds),
    // Item - fighter
    AddFighter(ICmdFighterAddFCtxRIds),
    ChangeFighter(ICmdFighterChangeFCtxRIds),
    // Item - fit-wide effect
    AddFwEffect(ICmdFwEffectAddFCtxRIds),
    ChangeFwEffect(ICmdFwEffectChangeFCtxRIds),
    // Item - implant
    AddImplant(ICmdImplantAddFCtxRIds),
    ChangeImplant(ICmdImplantChangeFCtxRIds),
    // Item - module
    AddModule(ICmdModuleAddFCtxRIds),
    ChangeModule(ICmdModuleChangeFCtxRIds),
    // Item - projected effect
    AddProjEffect(ICmdProjEffectAddFCtxRIds),
    ChangeProjEffect(ICmdProjEffectChangeFCtxRIds),
    // Item - rig
    AddRig(ICmdRigAddFCtxRIds),
    ChangeRig(ICmdRigChangeFCtxRIds),
    // Item - service
    AddService(ICmdServiceAddFCtxRIds),
    ChangeService(ICmdServiceChangeFCtxRIds),
    // Item - ship
    SetShip(ICmdShipSetFCtxRIds),
    ChangeShip(SolChangeShipCmdRIds),
    UnsetShip(ICmdShipUnsetFCtxRIds),
    // Item - skill
    AddSkill(ICmdSkillAddFCtxRIds),
    ChangeSkill(ICmdSkillChangeFCtxRIds),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeSolEnumCmd {
    pub(crate) fn render(self, resps: &CmdResps) -> Result<ChangeSolEnumCmdRIds, BackrefRenderError> {
        Ok(match self {
            // Solar system
            Self::ChangeSol(cmd) => ChangeSolEnumCmdRIds::ChangeSol(cmd.inner),
            // Fleet
            Self::AddFleet(cmd) => ChangeSolEnumCmdRIds::AddFleet(cmd.inner.render(resps)?),
            Self::ChangeFleet(cmd) => ChangeSolEnumCmdRIds::ChangeFleet(cmd.inner.render(resps)?),
            Self::RemoveFleet(cmd) => ChangeSolEnumCmdRIds::RemoveFleet(cmd.inner.render(resps)?),
            // Fit
            Self::AddFit(cmd) => ChangeSolEnumCmdRIds::AddFit(cmd.inner.render(resps)?),
            Self::ChangeFit(cmd) => ChangeSolEnumCmdRIds::ChangeFit(cmd.inner.render(resps)?),
            Self::RemoveFit(cmd) => ChangeSolEnumCmdRIds::RemoveFit(cmd.inner.render(resps)?),
            // Item
            Self::RemoveItem(cmd) => ChangeSolEnumCmdRIds::RemoveItem(cmd.inner.render(resps)?),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => ChangeSolEnumCmdRIds::ChangeAutocharge(cmd.inner.render(resps)?),
            // Item - booster
            Self::AddBooster(cmd) => ChangeSolEnumCmdRIds::AddBooster(cmd.inner.render(resps)?),
            Self::ChangeBooster(cmd) => ChangeSolEnumCmdRIds::ChangeBooster(cmd.inner.render(resps)?),
            // Item - character
            Self::SetCharacter(cmd) => ChangeSolEnumCmdRIds::SetCharacter(cmd.inner.render(resps)?),
            Self::ChangeCharacter(cmd) => ChangeSolEnumCmdRIds::ChangeCharacter(cmd.render(resps)?),
            Self::UnsetCharacter(cmd) => ChangeSolEnumCmdRIds::UnsetCharacter(cmd.inner.render(resps)?),
            // Item - charge
            Self::ChangeCharge(cmd) => ChangeSolEnumCmdRIds::ChangeCharge(cmd.inner.render(resps)?),
            // Item - drone
            Self::AddDrone(cmd) => ChangeSolEnumCmdRIds::AddDrone(cmd.inner.render(resps)?),
            Self::ChangeDrone(cmd) => ChangeSolEnumCmdRIds::ChangeDrone(cmd.inner.render(resps)?),
            // Item - fighter
            Self::AddFighter(cmd) => ChangeSolEnumCmdRIds::AddFighter(cmd.inner.render(resps)?),
            Self::ChangeFighter(cmd) => ChangeSolEnumCmdRIds::ChangeFighter(cmd.inner.render(resps)?),
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => ChangeSolEnumCmdRIds::AddFwEffect(cmd.inner.render(resps)?),
            Self::ChangeFwEffect(cmd) => ChangeSolEnumCmdRIds::ChangeFwEffect(cmd.inner.render(resps)?),
            // Item - implant
            Self::AddImplant(cmd) => ChangeSolEnumCmdRIds::AddImplant(cmd.inner.render(resps)?),
            Self::ChangeImplant(cmd) => ChangeSolEnumCmdRIds::ChangeImplant(cmd.inner.render(resps)?),
            // Item - module
            Self::AddModule(cmd) => ChangeSolEnumCmdRIds::AddModule(cmd.inner.render(resps)?),
            Self::ChangeModule(cmd) => ChangeSolEnumCmdRIds::ChangeModule(cmd.inner.render(resps)?),
            // Item - projected effect
            Self::AddProjEffect(cmd) => ChangeSolEnumCmdRIds::AddProjEffect(cmd.inner.render(resps)?),
            Self::ChangeProjEffect(cmd) => ChangeSolEnumCmdRIds::ChangeProjEffect(cmd.inner.render(resps)?),
            // Item - rig
            Self::AddRig(cmd) => ChangeSolEnumCmdRIds::AddRig(cmd.inner.render(resps)?),
            Self::ChangeRig(cmd) => ChangeSolEnumCmdRIds::ChangeRig(cmd.inner.render(resps)?),
            // Item - service
            Self::AddService(cmd) => ChangeSolEnumCmdRIds::AddService(cmd.inner.render(resps)?),
            Self::ChangeService(cmd) => ChangeSolEnumCmdRIds::ChangeService(cmd.inner.render(resps)?),
            // Item - ship
            Self::SetShip(cmd) => ChangeSolEnumCmdRIds::SetShip(cmd.inner.render(resps)?),
            Self::ChangeShip(cmd) => ChangeSolEnumCmdRIds::ChangeShip(cmd.render(resps)?),
            Self::UnsetShip(cmd) => ChangeSolEnumCmdRIds::UnsetShip(cmd.inner.render(resps)?),
            // Item - skill
            Self::AddSkill(cmd) => ChangeSolEnumCmdRIds::AddSkill(cmd.inner.render(resps)?),
            Self::ChangeSkill(cmd) => ChangeSolEnumCmdRIds::ChangeSkill(cmd.inner.render(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeSolEnumCmdRIds {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<CmdResp, ChangeSolEnumError> {
        match self {
            // Solar system
            Self::ChangeSol(cmd) => Ok(cmd.execute(core_sol).into()),
            // Fleet
            Self::AddFleet(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeFleet(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::RemoveFleet(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Fit
            Self::AddFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::RemoveFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item
            Self::RemoveItem(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - booster
            Self::AddBooster(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeBooster(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - character
            Self::SetCharacter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeCharacter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::UnsetCharacter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - charge
            Self::ChangeCharge(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - drone
            Self::AddDrone(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeDrone(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - fighter
            Self::AddFighter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeFighter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeFwEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - implant
            Self::AddImplant(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeImplant(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - module
            Self::AddModule(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeModule(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - projected effect
            Self::AddProjEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeProjEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - rig
            Self::AddRig(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeRig(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - service
            Self::AddService(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeService(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - ship
            Self::SetShip(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeShip(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::UnsetShip(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - skill
            Self::AddSkill(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeSkill(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeSolEnumError {
    // Fleet
    #[error("failed to add fleet: {0}")]
    FleetAddFailed(#[from] AddFleetError),
    #[error("failed to change fleet: {0}")]
    FleetChangeFailed(#[from] GetFleetChangeFleetError),
    #[error("failed to remove fleet: {0}")]
    FleetRemoveFailed(#[from] GetFleetRemoveFleetError),
    // Fit
    #[error("failed to add fit: {0}")]
    FitAddFailed(#[from] AddFitError),
    #[error("failed to change fit: {0}")]
    FitChangeFailed(#[from] GetFitChangeFitError),
    #[error("failed to remove fit: {0}")]
    FitRemoveFailed(#[from] GetFitRemoveFitError),
    // Item
    #[error("failed to remove item: {0}")]
    ItemRemoveFailed(#[from] GetItemRemoveItemError),
    // Item - autocharge
    #[error("failed to change autocharge: {0}")]
    AutochargeChangeFailed(#[from] GetItemChangeAutochargeError),
    // Item - booster
    #[error("failed to add booster: {0}")]
    BoosterAddFailed(#[from] GetFitAddBoosterError),
    #[error("failed to change booster: {0}")]
    BoosterChangeFailed(#[from] GetItemChangeBoosterError),
    // Item - character
    #[error("failed to set character: {0}")]
    CharacterSetFailed(#[from] GetFitSetCharacterError),
    #[error("failed to change character: {0}")]
    CharacterChangeFailed(#[from] ChangeCharacterError),
    #[error("failed to unset character: {0}")]
    CharacterUnsetFailed(#[from] GetFitUnsetCharacterError),
    // Item - charge
    #[error("failed to change charge: {0}")]
    ChargeChangeFailed(#[from] GetItemChangeChargeError),
    // Item - drone
    #[error("failed to add drone: {0}")]
    DroneAddFailed(#[from] GetFitAddDroneError),
    #[error("failed to change drone: {0}")]
    DroneChangeFailed(#[from] GetItemChangeDroneError),
    // Item - fighter
    #[error("failed to add fighter: {0}")]
    FighterAddFailed(#[from] GetFitAddFighterError),
    #[error("failed to change fighter: {0}")]
    FighterChangeFailed(#[from] GetItemChangeFighterError),
    // Item - fit-wide effect
    #[error("failed to add fit-wide effect: {0}")]
    FwEffectAddFailed(#[from] GetFitAddFwEffectError),
    #[error("failed to change fit-wide effect: {0}")]
    FwEffectChangeFailed(#[from] GetItemChangeFwEffectError),
    // Item - implant
    #[error("failed to add implant: {0}")]
    ImplantAddFailed(#[from] GetFitAddImplantError),
    #[error("failed to change implant: {0}")]
    ImplantChangeFailed(#[from] GetItemChangeImplantError),
    // Item - module
    #[error("failed to add module: {0}")]
    ModuleAddFailed(#[from] GetFitAddModuleError),
    #[error("failed to change module: {0}")]
    ModuleChangeFailed(#[from] GetItemChangeModuleError),
    // Item - projected effect
    #[error("failed to add projected effect: {0}")]
    ProjEffectAddFailed(#[from] AddProjEffectError),
    #[error("failed to change projected effect: {0}")]
    ProjEffectChangeFailed(#[from] GetItemChangeProjEffectError),
    // Item - rig
    #[error("failed to add rig: {0}")]
    RigAddFailed(#[from] GetFitAddRigError),
    #[error("failed to change rig: {0}")]
    RigChangeFailed(#[from] GetItemChangeRigError),
    // Item - service
    #[error("failed to add service: {0}")]
    ServiceAddFailed(#[from] GetFitAddServiceError),
    #[error("failed to change service: {0}")]
    ServiceChangeFailed(#[from] GetItemChangeServiceError),
    // Item - ship
    #[error("failed to set ship: {0}")]
    ShipSetFailed(#[from] GetFitSetShipError),
    #[error("failed to change ship: {0}")]
    ShipChangeFailed(#[from] ChangeShipError),
    #[error("failed to unset ship: {0}")]
    ShipUnsetFailed(#[from] GetFitUnsetShipError),
    // Item - skill
    #[error("failed to add skill: {0}")]
    SkillAddFailed(#[from] GetFitAddSkillError),
    #[error("failed to change skill: {0}")]
    SkillChangeFailed(#[from] GetItemChangeSkillError),
}
