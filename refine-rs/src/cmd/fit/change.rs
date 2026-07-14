use crate::cmd::{
    inner::{
        FitChangeFitError, GetItemChangeAutochargeError, GetItemChangeBoosterError, GetItemRemoveItemError,
        ICmdAutochargeChangeFCtxBIds, ICmdAutochargeChangeFCtxRIds, ICmdBoosterChangeFCtxBIds,
        ICmdBoosterChangeFCtxRIds, ICmdBoosterCreateICtx, ICmdFitChangeICtxBIds, ICmdFitChangeICtxRIds,
        ICmdItemRemoveFCtxBIds, ICmdItemRemoveFCtxRIds, ICmdRigCreateICtx,
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
    // Item - booster
    CreateBooster(FitCreateBoosterCmd),
    ChangeBooster(FitChangeBoosterCmd),
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
    // Item - booster
    CreateBooster(ICmdBoosterCreateICtx),
    ChangeBooster(ICmdBoosterChangeFCtxRIds),
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
            // Item - booster
            Self::CreateBooster(cmd) => ChangeFitEnumCmdRIds::CreateBooster(cmd.inner),
            Self::ChangeBooster(cmd) => ChangeFitEnumCmdRIds::ChangeBooster(cmd.inner.render(resps)?),
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
            // Item - booster
            Self::CreateBooster(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeBooster(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - rig
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
    // Item - booster
    #[error("failed to change booster: {0}")]
    BoosterChangeFailed(#[from] GetItemChangeBoosterError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - fit
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct FitChangeFitCmd {
    inner: ICmdFitChangeICtxBIds = ICmdFitChangeICtxBIds { .. },
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
            inner: ICmdItemRemoveFCtxBIds { item_id, .. },
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
            inner: ICmdAutochargeChangeFCtxBIds { item_id, .. },
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
// Sub-commands - item - booster
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct FitCreateBoosterCmd {
    inner: ICmdBoosterCreateICtx,
}
impl FitCreateBoosterCmd {
    pub fn new(type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdBoosterCreateICtx { type_id, .. },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.state = Some(state);
        self
    }
    pub fn with_side_effects(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, bool)>) -> Self {
        self.inner.side_effects.clear();
        self.inner.side_effects.extend(effect_modes);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.effect_modes.clear();
        self.inner.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitCreateBoosterCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitCreateBoosterCmd) -> Self {
        Self::CreateBooster(sub_cmd)
    }
}

pub struct FitChangeBoosterCmd {
    inner: ICmdBoosterChangeFCtxBIds,
}
impl FitChangeBoosterCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdBoosterChangeFCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.ictx_cmd.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_side_effects(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, bool)>) -> Self {
        self.inner.ictx_cmd.side_effects.clear();
        self.inner.ictx_cmd.side_effects.extend(effect_modes);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<FitChangeBoosterCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeBoosterCmd) -> Self {
        Self::ChangeBooster(sub_cmd)
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
