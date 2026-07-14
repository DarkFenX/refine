use crate::cmd::{
    FitAddBoosterCmd, FitAddRigCmd, FitChangeAutochargeCmd, FitChangeBoosterCmd, FitChangeCharacterCmd,
    FitChangeCharacterError, FitChangeChargeCmd, FitChangeFitCmd, FitChangeFitError, FitRemoveItemCmd,
    FitSetCharacterCmd, FitUnsetCharacterCmd, GetItemChangeAutochargeError, GetItemChangeBoosterError,
    GetItemChangeChargeError, GetItemRemoveItemError,
    inner::{
        ICmdAutochargeChangeFCtxRIds, ICmdBoosterAddICtx, ICmdBoosterChangeFCtxRIds, ICmdCharacterChangeICtx,
        ICmdCharacterSetICtx, ICmdCharacterUnsetICtx, ICmdChargeChangeFCtxRIds, ICmdFitChangeICtxRIds,
        ICmdItemRemoveFCtxRIds, ICmdRigAddICtx,
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
    // Item - rig
    AddRig(FitAddRigCmd),
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
    // Item - rig
    AddRig(ICmdRigAddICtx),
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
            // Item - rig
            Self::AddRig(cmd) => ChangeFitEnumCmdRIds::AddRig(cmd.inner),
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
            // Item - rig
            Self::AddRig(cmd) => Ok(cmd.execute(core_fit).into()),
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
    // Item - character
    #[error("failed to change charge: {0}")]
    ChargeChangeFailed(#[from] GetItemChangeChargeError),
}
