use crate::cmd::{
    inner::{
        CreateFitError, CreateFleetError, GetFitChangeFitError, GetFitCreateBoosterError, GetFitCreateRigError,
        GetFitRemoveFitError, GetFleetChangeFleetError, GetFleetRemoveFleetError, GetItemChangeAutochargeError,
        GetItemChangeBoosterError, GetItemRemoveItemError, ICmdAutochargeChangeFCtxBIds, ICmdAutochargeChangeFCtxRIds,
        ICmdBoosterChangeFCtxBIds, ICmdBoosterChangeFCtxRIds, ICmdBoosterCreateFCtxBIds, ICmdBoosterCreateFCtxRIds,
        ICmdBoosterCreateICtx, ICmdFitChangeFCtxBIds, ICmdFitChangeFCtxRIds, ICmdFitCreateFCtxBIds,
        ICmdFitCreateFCtxRIds, ICmdFitRemoveFCtxBIds, ICmdFitRemoveFCtxRIds, ICmdFleetChangeFCtxBIds,
        ICmdFleetChangeFCtxRIds, ICmdFleetCreateFCtxBIds, ICmdFleetCreateFCtxRIds, ICmdFleetRemoveFCtxBIds,
        ICmdFleetRemoveFCtxRIds, ICmdItemRemoveFCtxBIds, ICmdItemRemoveFCtxRIds, ICmdRigCreateFCtxBIds,
        ICmdRigCreateFCtxRIds, ICmdRigCreateICtx, ICmdSolChangeFCtx,
    },
    shared::{BackrefRenderError, CmdResp, CmdResps, FitIdBackref, FleetIdBackref, ItemIdBackref},
};

pub enum ChangeSolEnumCmd {
    // Solar system
    ChangeSol(SolChangeSolCmd),
    // Fleet
    CreateFleet(SolCreateFleetCmd),
    ChangeFleet(SolChangeFleetCmd),
    RemoveFleet(SolRemoveFleetCmd),
    // Fit
    CreateFit(SolCreateFitCmd),
    ChangeFit(SolChangeFitCmd),
    RemoveFit(SolRemoveFitCmd),
    // Item
    RemoveItem(SolRemoveItemCmd),
    // Item - autocharge
    ChangeAutocharge(SolChangeAutochargeCmd),
    // Item - booster
    CreateBooster(SolCreateBoosterCmd),
    ChangeBooster(SolChangeBoosterCmd),
    // Item - rig
    CreateRig(SolCreateRigCmd),
}

pub(crate) enum ChangeSolEnumCmdRIds {
    // Solar system
    ChangeSol(ICmdSolChangeFCtx),
    // Fleet
    CreateFleet(ICmdFleetCreateFCtxRIds),
    ChangeFleet(ICmdFleetChangeFCtxRIds),
    RemoveFleet(ICmdFleetRemoveFCtxRIds),
    // Fit
    CreateFit(ICmdFitCreateFCtxRIds),
    ChangeFit(ICmdFitChangeFCtxRIds),
    RemoveFit(ICmdFitRemoveFCtxRIds),
    // Item
    RemoveItem(ICmdItemRemoveFCtxRIds),
    // Item - autocharge
    ChangeAutocharge(ICmdAutochargeChangeFCtxRIds),
    // Item - booster
    CreateBooster(ICmdBoosterCreateFCtxRIds),
    ChangeBooster(ICmdBoosterChangeFCtxRIds),
    // Item - rig
    CreateRig(ICmdRigCreateFCtxRIds),
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
            Self::CreateFleet(cmd) => ChangeSolEnumCmdRIds::CreateFleet(cmd.inner.render(resps)?),
            Self::ChangeFleet(cmd) => ChangeSolEnumCmdRIds::ChangeFleet(cmd.inner.render(resps)?),
            Self::RemoveFleet(cmd) => ChangeSolEnumCmdRIds::RemoveFleet(cmd.inner.render(resps)?),
            // Fit
            Self::CreateFit(cmd) => ChangeSolEnumCmdRIds::CreateFit(cmd.inner.render(resps)?),
            Self::ChangeFit(cmd) => ChangeSolEnumCmdRIds::ChangeFit(cmd.inner.render(resps)?),
            Self::RemoveFit(cmd) => ChangeSolEnumCmdRIds::RemoveFit(cmd.inner.render(resps)?),
            // Item
            Self::RemoveItem(cmd) => ChangeSolEnumCmdRIds::RemoveItem(cmd.inner.render(resps)?),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => ChangeSolEnumCmdRIds::ChangeAutocharge(cmd.inner.render(resps)?),
            // Item - booster
            Self::CreateBooster(cmd) => ChangeSolEnumCmdRIds::CreateBooster(cmd.inner.render(resps)?),
            Self::ChangeBooster(cmd) => ChangeSolEnumCmdRIds::ChangeBooster(cmd.inner.render(resps)?),
            // Item - rig
            Self::CreateRig(cmd) => ChangeSolEnumCmdRIds::CreateRig(cmd.inner.render(resps)?),
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
            Self::CreateFleet(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeFleet(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::RemoveFleet(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Fit
            Self::CreateFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::RemoveFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item
            Self::RemoveItem(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - booster
            Self::CreateBooster(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeBooster(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - rig
            Self::CreateRig(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeSolEnumError {
    // Fleet
    #[error("failed to create fleet: {0}")]
    FleetCreateFailed(#[from] CreateFleetError),
    #[error("failed to change fleet: {0}")]
    FleetChangeFailed(#[from] GetFleetChangeFleetError),
    #[error("failed to remove fleet: {0}")]
    FleetRemoveFailed(#[from] GetFleetRemoveFleetError),
    // Fit
    #[error("failed to create fit: {0}")]
    FitCreateFailed(#[from] CreateFitError),
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
    #[error("failed to create booster: {0}")]
    BoosterCreateFailed(#[from] GetFitCreateBoosterError),
    #[error("failed to change booster: {0}")]
    BoosterChangeFailed(#[from] GetItemChangeBoosterError),
    // Item - rig
    #[error("failed to create rig: {0}")]
    RigCreateFailed(#[from] GetFitCreateRigError),
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
pub struct SolCreateFleetCmd {
    inner: ICmdFleetCreateFCtxBIds = ICmdFleetCreateFCtxBIds { .. },
}
impl SolCreateFleetCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit_ids(mut self, fit_ids: impl Iterator<Item = FitIdBackref>) -> Self {
        self.inner.fit_ids.clear();
        self.inner.fit_ids.extend(fit_ids);
        self
    }
}
impl From<SolCreateFleetCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolCreateFleetCmd) -> Self {
        Self::CreateFleet(sub_cmd)
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
pub struct SolCreateFitCmd {
    inner: ICmdFitCreateFCtxBIds = ICmdFitCreateFCtxBIds { .. },
}
impl SolCreateFitCmd {
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
impl From<SolCreateFitCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolCreateFitCmd) -> Self {
        Self::CreateFit(sub_cmd)
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
pub struct SolCreateBoosterCmd {
    inner: ICmdBoosterCreateFCtxBIds,
}
impl SolCreateBoosterCmd {
    pub fn new(fit_id: FitIdBackref, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdBoosterCreateFCtxBIds {
                fit_id,
                ictx_cmd: ICmdBoosterCreateICtx { type_id, .. },
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
impl From<SolCreateBoosterCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolCreateBoosterCmd) -> Self {
        Self::CreateBooster(sub_cmd)
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
pub struct SolCreateRigCmd {
    inner: ICmdRigCreateFCtxBIds,
}
impl SolCreateRigCmd {
    pub fn new(fit_id: FitIdBackref, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdRigCreateFCtxBIds {
                fit_id,
                ictx_cmd: ICmdRigCreateICtx { type_id, .. },
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
impl From<SolCreateRigCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolCreateRigCmd) -> Self {
        Self::CreateRig(sub_cmd)
    }
}
