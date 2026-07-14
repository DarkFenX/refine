use crate::cmd::{
    inner::{
        CmdAutochargeChangeFCtxBIds, CmdAutochargeChangeFCtxRIds, CmdAutochargeChangeICtx, CmdFitChangeFCtxBIds,
        CmdFitChangeFCtxRIds, CmdFitChangeICtxBIds, CmdFitCreateFCtxBIds, CmdFitCreateFCtxRIds, CmdFitRemoveFCtxBIds,
        CmdFitRemoveFCtxRIds, CmdFitRemoveICtx, CmdFleetChangeFCtxBIds, CmdFleetChangeFCtxRIds, CmdFleetChangeICtxBIds,
        CmdFleetCreateFCtxBIds, CmdFleetCreateFCtxRIds, CmdFleetRemoveFCtxBIds, CmdFleetRemoveFCtxRIds,
        CmdFleetRemoveICtx, CmdItemRemoveFCtxBIds, CmdItemRemoveFCtxRIds, CmdItemRemoveICtx, CmdSolChangeFCtx,
        CreateFitError, CreateFleetError, GetFitChangeFitError, GetFitRemoveFitError, GetFleetChangeFleetError,
        GetFleetRemoveFleetError, GetItemChangeAutochargeError, GetItemRemoveItemError,
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
}

pub(crate) enum ChangeSolEnumCmdRIds {
    // Solar system
    ChangeSol(CmdSolChangeFCtx),
    // Fleet
    CreateFleet(CmdFleetCreateFCtxRIds),
    ChangeFleet(CmdFleetChangeFCtxRIds),
    RemoveFleet(CmdFleetRemoveFCtxRIds),
    // Fit
    CreateFit(CmdFitCreateFCtxRIds),
    ChangeFit(CmdFitChangeFCtxRIds),
    RemoveFit(CmdFitRemoveFCtxRIds),
    // Item
    RemoveItem(CmdItemRemoveFCtxRIds),
    // Item - autocharge
    ChangeAutocharge(CmdAutochargeChangeFCtxRIds),
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
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - solar system
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct SolChangeSolCmd {
    inner: CmdSolChangeFCtx,
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
    inner: CmdFleetCreateFCtxBIds,
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
    inner: CmdFleetChangeFCtxBIds,
}
impl SolChangeFleetCmd {
    pub fn new(fleet_id: FleetIdBackref) -> Self {
        Self {
            inner: CmdFleetChangeFCtxBIds {
                fleet_id,
                ictx_cmd: CmdFleetChangeICtxBIds::default(),
            },
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
    inner: CmdFleetRemoveFCtxBIds,
}
impl SolRemoveFleetCmd {
    pub fn new(fleet_id: FleetIdBackref) -> Self {
        Self {
            inner: CmdFleetRemoveFCtxBIds {
                fleet_id,
                ictx_cmd: CmdFleetRemoveICtx::default(),
            },
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
    inner: CmdFitCreateFCtxBIds,
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
    inner: CmdFitChangeFCtxBIds,
}
impl SolChangeFitCmd {
    pub fn new(fit_id: FitIdBackref) -> Self {
        Self {
            inner: CmdFitChangeFCtxBIds {
                fit_id,
                ictx_cmd: CmdFitChangeICtxBIds::default(),
            },
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
    inner: CmdFitRemoveFCtxBIds,
}
impl SolRemoveFitCmd {
    pub fn new(fit_id: FitIdBackref) -> Self {
        Self {
            inner: CmdFitRemoveFCtxBIds {
                fit_id,
                ictx_cmd: CmdFitRemoveICtx::default(),
            },
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
    inner: CmdItemRemoveFCtxBIds,
}
impl SolRemoveItemCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: CmdItemRemoveFCtxBIds {
                item_id,
                ictx_cmd: CmdItemRemoveICtx::default(),
            },
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
    inner: CmdAutochargeChangeFCtxBIds,
}
impl SolChangeAutochargeCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: CmdAutochargeChangeFCtxBIds {
                item_id,
                ictx_cmd: CmdAutochargeChangeICtx::default(),
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
impl From<SolChangeAutochargeCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeAutochargeCmd) -> Self {
        Self::ChangeAutocharge(sub_cmd)
    }
}
