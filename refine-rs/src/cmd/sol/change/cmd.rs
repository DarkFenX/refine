use super::sub_item_character::SolChangeCharacterCmdRIds;
use crate::cmd::{
    AddFitError, AddFleetError, ChangeCharacterError, GetFitAddBoosterError, GetFitAddDroneError, GetFitAddRigError,
    GetFitChangeFitError, GetFitRemoveFitError, GetFitSetCharacterError, GetFitUnsetCharacterError,
    GetFleetChangeFleetError, GetFleetRemoveFleetError, GetItemChangeAutochargeError, GetItemChangeBoosterError,
    GetItemChangeChargeError, GetItemChangeDroneError, GetItemChangeRigError, GetItemRemoveItemError, SolAddBoosterCmd,
    SolAddDroneCmd, SolAddFitCmd, SolAddFleetCmd, SolAddRigCmd, SolChangeAutochargeCmd, SolChangeBoosterCmd,
    SolChangeCharacterCmd, SolChangeChargeCmd, SolChangeDroneCmd, SolChangeFitCmd, SolChangeFleetCmd, SolChangeRigCmd,
    SolChangeSolCmd, SolRemoveFitCmd, SolRemoveFleetCmd, SolRemoveItemCmd, SolSetCharacterCmd, SolUnsetCharacterCmd,
    inner::{
        ICmdAutochargeChangeFCtxRIds, ICmdBoosterAddFCtxRIds, ICmdBoosterChangeFCtxRIds, ICmdCharacterSetFCtxRIds,
        ICmdCharacterUnsetFCtxRIds, ICmdChargeChangeFCtxRIds, ICmdDroneAddFCtxRIds, ICmdDroneChangeFCtxRIds,
        ICmdFitAddFCtxRIds, ICmdFitChangeFCtxRIds, ICmdFitRemoveFCtxRIds, ICmdFleetAddFCtxRIds,
        ICmdFleetChangeFCtxRIds, ICmdFleetRemoveFCtxRIds, ICmdItemRemoveFCtxRIds, ICmdRigAddFCtxRIds,
        ICmdRigChangeFCtxRIds, ICmdSolChangeFCtx,
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
    // Item - rig
    AddRig(SolAddRigCmd),
    ChangeRig(SolChangeRigCmd),
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
    // Item - rig
    AddRig(ICmdRigAddFCtxRIds),
    ChangeRig(ICmdRigChangeFCtxRIds),
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
            // Item - booster
            Self::AddDrone(cmd) => ChangeSolEnumCmdRIds::AddDrone(cmd.inner.render(resps)?),
            Self::ChangeDrone(cmd) => ChangeSolEnumCmdRIds::ChangeDrone(cmd.inner.render(resps)?),
            // Item - rig
            Self::AddRig(cmd) => ChangeSolEnumCmdRIds::AddRig(cmd.inner.render(resps)?),
            Self::ChangeRig(cmd) => ChangeSolEnumCmdRIds::ChangeRig(cmd.inner.render(resps)?),
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
            // Item - rig
            Self::AddRig(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeRig(cmd) => Ok(cmd.execute(core_sol)?.into()),
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
    // Item - rig
    #[error("failed to add rig: {0}")]
    RigAddFailed(#[from] GetFitAddRigError),
    #[error("failed to change rig: {0}")]
    RigChangeFailed(#[from] GetItemChangeRigError),
}
