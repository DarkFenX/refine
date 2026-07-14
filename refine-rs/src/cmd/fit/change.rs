use crate::cmd::{
    inner::{
        FitChangeFitError, GetItemChangeAutochargeError, GetItemRemoveItemError, ICmdAutochargeChangeFCtxBIds,
        ICmdAutochargeChangeFCtxRIds, ICmdAutochargeChangeICtx, ICmdFitChangeICtxBIds, ICmdFitChangeICtxRIds,
        ICmdItemRemoveFCtxBIds, ICmdItemRemoveFCtxRIds, ICmdItemRemoveICtx, ICmdRigCreateICtx,
    },
    shared::{BackrefRenderError, CmdResp, CmdResps, FleetIdBackref, ItemIdBackref},
};

pub enum ChangeFitEnumCmd {
    // Fit
    ChangeFit(FitChangeFitCmd),
    // Item
    RemoveItem(FitRemoveItemCmd),
    // Item - autocharge
    ChangeAutocharge(FitChangeAutochargeCmd),
    // Item - rig
    CreateRig(FitCreateRigCmd),
}

pub(crate) enum ChangeFitEnumCmdRIds {
    // Fit
    ChangeFit(ICmdFitChangeICtxRIds),
    // Item
    RemoveItem(ICmdItemRemoveFCtxRIds),
    // Item - autocharge
    ChangeAutocharge(ICmdAutochargeChangeFCtxRIds),
    // Item - rig
    CreateRig(ICmdRigCreateICtx),
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
            // Item - rig
            Self::CreateRig(cmd) => ChangeFitEnumCmdRIds::CreateRig(cmd.inner),
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
            // Item - autocharge
            Self::CreateRig(cmd) => Ok(cmd.execute(core_fit).into()),
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
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - fit
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct FitChangeFitCmd {
    inner: ICmdFitChangeICtxBIds,
}
impl FitChangeFitCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_id(mut self, fleet_id: Option<FleetIdBackref>) -> Self {
        self.inner.fleet_id = fleet_id.into();
        self
    }
    pub fn with_sec_status(mut self, sec_status: rc::FitSecStatus) -> Self {
        self.inner.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: Option<rc::DpsProfile>) -> Self {
        self.inner.shared.rah_incoming_dps = rah_incoming_dps.into();
        self
    }
}
impl From<FitChangeFitCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeFitCmd) -> Self {
        Self::ChangeFit(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - item
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct FitRemoveItemCmd {
    inner: ICmdItemRemoveFCtxBIds,
}
impl FitRemoveItemCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdItemRemoveFCtxBIds {
                item_id,
                ictx_cmd: ICmdItemRemoveICtx::default(),
            },
        }
    }
    pub fn with_rm_mode(mut self, rm_mode: rc::RmMode) -> Self {
        self.inner.ictx_cmd.rm_mode = Some(rm_mode);
        self
    }
}
impl From<FitRemoveItemCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitRemoveItemCmd) -> Self {
        Self::RemoveItem(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - item - autocharge
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct FitChangeAutochargeCmd {
    inner: ICmdAutochargeChangeFCtxBIds,
}
impl FitChangeAutochargeCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdAutochargeChangeFCtxBIds {
                item_id,
                ictx_cmd: ICmdAutochargeChangeICtx::default(),
            },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitChangeAutochargeCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeAutochargeCmd) -> Self {
        Self::ChangeAutocharge(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - item - rig
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct FitCreateRigCmd {
    inner: ICmdRigCreateICtx,
}
impl FitCreateRigCmd {
    pub fn new(type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdRigCreateICtx { type_id, .. },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.effect_modes.clear();
        self.inner.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitCreateRigCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitCreateRigCmd) -> Self {
        Self::CreateRig(sub_cmd)
    }
}
