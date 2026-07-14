use crate::cmd::{
    inner::{
        AddFitError, AddFleetError, GetFitAddBoosterError, GetFitAddRigError, GetFitChangeFitError,
        GetFitRemoveFitError, GetFleetChangeFleetError, GetFleetRemoveFleetError, GetItemChangeAutochargeError,
        GetItemChangeBoosterError, GetItemRemoveItemError, ICmdAutochargeChangeFCtxBIds, ICmdAutochargeChangeFCtxRIds,
        ICmdBoosterAddFCtxBIds, ICmdBoosterAddFCtxRIds, ICmdBoosterAddICtx, ICmdBoosterChangeFCtxBIds,
        ICmdBoosterChangeFCtxRIds, ICmdFitAddFCtxBIds, ICmdFitAddFCtxRIds, ICmdFitChangeFCtxBIds,
        ICmdFitChangeFCtxRIds, ICmdFitRemoveFCtxBIds, ICmdFitRemoveFCtxRIds, ICmdFleetAddFCtxBIds,
        ICmdFleetAddFCtxRIds, ICmdFleetChangeFCtxBIds, ICmdFleetChangeFCtxRIds, ICmdFleetRemoveFCtxBIds,
        ICmdFleetRemoveFCtxRIds, ICmdItemRemoveFCtxBIds, ICmdItemRemoveFCtxRIds, ICmdRigAddFCtxBIds,
        ICmdRigAddFCtxRIds, ICmdRigAddICtx, ICmdSolChangeFCtx,
    },
    shared::{BackrefRenderError, CmdResp, CmdResps, FitIdBackref, FleetIdBackref, ItemIdBackref},
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
    // Item - rig
    AddRig(SolAddRigCmd),
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
    // Item - rig
    AddRig(ICmdRigAddFCtxRIds),
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
            // Item - rig
            Self::AddRig(cmd) => ChangeSolEnumCmdRIds::AddRig(cmd.inner.render(resps)?),
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
            // Item - rig
            Self::AddRig(cmd) => Ok(cmd.execute(core_sol)?.into()),
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
    // Item - rig
    #[error("failed to add rig: {0}")]
    RigAddFailed(#[from] GetFitAddRigError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - solar system
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct SolChangeSolCmd {
    inner: ICmdSolChangeFCtx = ICmdSolChangeFCtx { .. },
}
impl SolChangeSolCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_sec_zone(mut self, sec_zone: rc::SecZone) -> Self {
        self.inner.sec_zone = Some(sec_zone);
        self
    }
    pub fn with_default_incoming_dps(mut self, incoming_dps: rc::DpsProfile) -> Self {
        self.inner.default_incoming_dps = Some(incoming_dps);
        self
    }
    pub fn with_default_spool(mut self, spool: rc::Spool) -> Self {
        self.inner.default_spool = Some(spool);
        self
    }
    pub fn with_default_npc_prop(mut self, npc_prop: rc::NpcProp) -> Self {
        self.inner.default_npc_prop = Some(npc_prop);
        self
    }
    pub fn with_default_optional_reloads(mut self, optional_reload: rc::OptionalReload) -> Self {
        self.inner.default_optional_reloads = Some(optional_reload);
        self
    }
    pub fn with_default_rearm_minions(mut self, rearm_minion: rc::RearmMinion) -> Self {
        self.inner.default_rearm_minions = Some(rearm_minion);
        self
    }
}
impl From<SolChangeSolCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeSolCmd) -> Self {
        Self::ChangeSol(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - fleet
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct SolAddFleetCmd {
    inner: ICmdFleetAddFCtxBIds = ICmdFleetAddFCtxBIds { .. },
}
impl SolAddFleetCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit_ids(mut self, fit_ids: impl Iterator<Item = FitIdBackref>) -> Self {
        self.inner.fit_ids.clear();
        self.inner.fit_ids.extend(fit_ids);
        self
    }
}
impl From<SolAddFleetCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddFleetCmd) -> Self {
        Self::AddFleet(sub_cmd)
    }
}

pub struct SolChangeFleetCmd {
    inner: ICmdFleetChangeFCtxBIds,
}
impl SolChangeFleetCmd {
    pub fn new(fleet_id: FleetIdBackref) -> Self {
        Self {
            inner: ICmdFleetChangeFCtxBIds { fleet_id, .. },
        }
    }
    pub fn with_add_fit_ids(mut self, add_fit_ids: impl Iterator<Item = FitIdBackref>) -> Self {
        self.inner.ictx_cmd.add_fit_ids.clear();
        self.inner.ictx_cmd.add_fit_ids.extend(add_fit_ids);
        self
    }
    pub fn with_rm_fit_ids(mut self, rm_fit_ids: impl Iterator<Item = FitIdBackref>) -> Self {
        self.inner.ictx_cmd.rm_fit_ids.clear();
        self.inner.ictx_cmd.rm_fit_ids.extend(rm_fit_ids);
        self
    }
}
impl From<SolChangeFleetCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeFleetCmd) -> Self {
        Self::ChangeFleet(sub_cmd)
    }
}

pub struct SolRemoveFleetCmd {
    inner: ICmdFleetRemoveFCtxBIds,
}
impl SolRemoveFleetCmd {
    pub fn new(fleet_id: FleetIdBackref) -> Self {
        Self {
            inner: ICmdFleetRemoveFCtxBIds { fleet_id, .. },
        }
    }
}
impl From<SolRemoveFleetCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolRemoveFleetCmd) -> Self {
        Self::RemoveFleet(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - fit
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct SolAddFitCmd {
    inner: ICmdFitAddFCtxBIds = ICmdFitAddFCtxBIds { .. },
}
impl SolAddFitCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_id(mut self, fleet_id: FleetIdBackref) -> Self {
        self.inner.fleet_id = Some(fleet_id);
        self
    }
    pub fn with_sec_status(mut self, sec_status: rc::FitSecStatus) -> Self {
        self.inner.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: rc::DpsProfile) -> Self {
        self.inner.shared.rah_incoming_dps = Some(rah_incoming_dps);
        self
    }
}
impl From<SolAddFitCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddFitCmd) -> Self {
        Self::AddFit(sub_cmd)
    }
}

pub struct SolChangeFitCmd {
    inner: ICmdFitChangeFCtxBIds,
}
impl SolChangeFitCmd {
    pub fn new(fit_id: FitIdBackref) -> Self {
        Self {
            inner: ICmdFitChangeFCtxBIds { fit_id, .. },
        }
    }
    pub fn with_fleet_id(mut self, fleet_id: Option<FleetIdBackref>) -> Self {
        self.inner.ictx_cmd.fleet_id = fleet_id.into();
        self
    }
    pub fn with_sec_status(mut self, sec_status: rc::FitSecStatus) -> Self {
        self.inner.ictx_cmd.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: Option<rc::DpsProfile>) -> Self {
        self.inner.ictx_cmd.shared.rah_incoming_dps = rah_incoming_dps.into();
        self
    }
}
impl From<SolChangeFitCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeFitCmd) -> Self {
        Self::ChangeFit(sub_cmd)
    }
}

pub struct SolRemoveFitCmd {
    inner: ICmdFitRemoveFCtxBIds,
}
impl SolRemoveFitCmd {
    pub fn new(fit_id: FitIdBackref) -> Self {
        Self {
            inner: ICmdFitRemoveFCtxBIds { fit_id, .. },
        }
    }
}
impl From<SolRemoveFitCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolRemoveFitCmd) -> Self {
        Self::RemoveFit(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - item
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SolRemoveItemCmd {
    inner: ICmdItemRemoveFCtxBIds,
}
impl SolRemoveItemCmd {
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
impl From<SolRemoveItemCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolRemoveItemCmd) -> Self {
        Self::RemoveItem(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - item - autocharge
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SolChangeAutochargeCmd {
    inner: ICmdAutochargeChangeFCtxBIds,
}
impl SolChangeAutochargeCmd {
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
impl From<SolChangeAutochargeCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeAutochargeCmd) -> Self {
        Self::ChangeAutocharge(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - item - booster
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SolAddBoosterCmd {
    inner: ICmdBoosterAddFCtxBIds,
}
impl SolAddBoosterCmd {
    pub fn new(fit_id: FitIdBackref, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdBoosterAddFCtxBIds {
                fit_id,
                ictx_cmd: ICmdBoosterAddICtx { type_id, .. },
            },
        }
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
impl From<SolAddBoosterCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddBoosterCmd) -> Self {
        Self::AddBooster(sub_cmd)
    }
}

pub struct SolChangeBoosterCmd {
    inner: ICmdBoosterChangeFCtxBIds,
}
impl SolChangeBoosterCmd {
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
impl From<SolChangeBoosterCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeBoosterCmd) -> Self {
        Self::ChangeBooster(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - item - rig
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SolAddRigCmd {
    inner: ICmdRigAddFCtxBIds,
}
impl SolAddRigCmd {
    pub fn new(fit_id: FitIdBackref, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdRigAddFCtxBIds {
                fit_id,
                ictx_cmd: ICmdRigAddICtx { type_id, .. },
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
impl From<SolAddRigCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddRigCmd) -> Self {
        Self::AddRig(sub_cmd)
    }
}
