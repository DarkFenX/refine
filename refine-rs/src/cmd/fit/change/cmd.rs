use crate::cmd::{
    FitAddBoosterCmd, FitAddDroneCmd, FitAddDroneError, FitAddFighterCmd, FitAddFighterError, FitAddFwEffectCmd,
    FitAddImplantCmd, FitAddModuleCmd, FitAddModuleError, FitAddRigCmd, FitChangeAutochargeCmd, FitChangeBoosterCmd,
    FitChangeCharacterCmd, FitChangeCharacterError, FitChangeChargeCmd, FitChangeDroneCmd, FitChangeFighterCmd,
    FitChangeFitCmd, FitChangeFitError, FitChangeFwEffectCmd, FitChangeImplantCmd, FitChangeModuleCmd, FitChangeRigCmd,
    FitRemoveItemCmd, FitSetCharacterCmd, FitUnsetCharacterCmd, GetItemChangeAutochargeError,
    GetItemChangeBoosterError, GetItemChangeChargeError, GetItemChangeDroneError, GetItemChangeFighterError,
    GetItemChangeFwEffectError, GetItemChangeImplantError, GetItemChangeModuleError, GetItemChangeRigError,
    GetItemRemoveItemError,
    inner::{
        ICmdAutochargeChangeFCtxRIds, ICmdBoosterAddICtx, ICmdBoosterChangeFCtxRIds, ICmdCharacterChangeICtx,
        ICmdCharacterSetICtx, ICmdCharacterUnsetICtx, ICmdChargeChangeFCtxRIds, ICmdDroneAddICtxRIds,
        ICmdDroneChangeFCtxRIds, ICmdFighterAddICtxRIds, ICmdFighterChangeFCtxRIds, ICmdFitChangeICtxRIds,
        ICmdFwEffectAddICtx, ICmdFwEffectChangeFCtxRIds, ICmdImplantAddICtx, ICmdImplantChangeFCtxRIds,
        ICmdItemRemoveFCtxRIds, ICmdModuleAddICtxRIds, ICmdModuleChangeFCtxRIds, ICmdRigAddICtx, ICmdRigChangeFCtxRIds,
    },
    shared::{BackrefRenderError, CmdResp, CmdResps},
};

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
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeFitEnumCmd {
    pub(crate) fn render(self, resps: &CmdResps) -> Result<ChangeFitEnumCmdRIds, BackrefRenderError> {
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
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeFitEnumCmdRIds {
    pub(crate) fn execute(&self, core_fit: &mut rc::FitMut) -> Result<CmdResp, ChangeFitEnumError> {
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
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeFitEnumError {
    // Fit
    #[error("failed to change fleet: {0}")]
    FitChangeFailed(#[from] FitChangeFitError),
    // Item
    #[error("failed to remove item: {0}")]
    ItemRemoveFailed(#[from] GetItemRemoveItemError),
    // Item - autocharge
    #[error("failed to change autocharge: {0}")]
    AutochargeChangeFailed(#[from] GetItemChangeAutochargeError),
    // Item - booster
    #[error("failed to change booster: {0}")]
    BoosterChangeFailed(#[from] GetItemChangeBoosterError),
    // Item - character
    #[error("failed to change character: {0}")]
    CharacterChangeFailed(#[from] FitChangeCharacterError),
    // Item - charge
    #[error("failed to change charge: {0}")]
    ChargeChangeFailed(#[from] GetItemChangeChargeError),
    // Item - drone
    #[error("failed to add drone: {0}")]
    DroneAddFailed(#[from] FitAddDroneError),
    #[error("failed to change drone: {0}")]
    DroneChangeFailed(#[from] GetItemChangeDroneError),
    // Item - fighter
    #[error("failed to add fighter: {0}")]
    FighterAddFailed(#[from] FitAddFighterError),
    #[error("failed to change fighter: {0}")]
    FighterChangeFailed(#[from] GetItemChangeFighterError),
    // Item - fit-wide effect
    #[error("failed to change fit-wide effect: {0}")]
    FwEffectChangeFailed(#[from] GetItemChangeFwEffectError),
    // Item - implant
    #[error("failed to change implant: {0}")]
    ImplantChangeFailed(#[from] GetItemChangeImplantError),
    // Item - module
    #[error("failed to add module: {0}")]
    ModuleAddFailed(#[from] FitAddModuleError),
    #[error("failed to change module: {0}")]
    ModuleChangeFailed(#[from] GetItemChangeModuleError),
    // Item - rig
    #[error("failed to change rig: {0}")]
    RigChangeFailed(#[from] GetItemChangeRigError),
}
