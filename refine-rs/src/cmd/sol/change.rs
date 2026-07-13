use crate::cmd::{
    BackrefRenderError, CmdResp, CmdResps, CreateFleetError, FitIdBackref, FleetIdBackref, GetFitChangeFitError,
    GetFleetChangeFleetError, GetFleetRemoveFleetError,
    basic::{
        CmdFitChangeFCtxBIds, CmdFitChangeFCtxRIds, CmdFitChangeICtxBIds, CmdFleetChangeFCtxBIds,
        CmdFleetChangeFCtxRIds, CmdFleetChangeICtxBIds, CmdFleetCreateFCtxBIds, CmdFleetCreateFCtxRIds,
        CmdFleetRemoveFCtxBIds, CmdFleetRemoveFCtxRIds, CmdFleetRemoveICtx, CmdSolChangeFCtx,
    },
};

pub enum ChangeSolEnumCmd {
    // Solar system
    ChangeSol(SolChangeSolCmd),
    // Fleet
    CreateFleet(SolCreateFleetCmd),
    ChangeFleet(SolChangeFleetCmd),
    RemoveFleet(SolRemoveFleetCmd),
    // Fit
    ChangeFit(SolChangeFitCmd),
}

pub(crate) enum ChangeSolEnumCmdRIds {
    // Solar system
    ChangeSol(CmdSolChangeFCtx),
    // Fleet
    CreateFleet(CmdFleetCreateFCtxRIds),
    ChangeFleet(CmdFleetChangeFCtxRIds),
    RemoveFleet(CmdFleetRemoveFCtxRIds),
    // Fit
    ChangeFit(CmdFitChangeFCtxRIds),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeSolEnumCmd {
    pub(crate) fn render(self, resps: &CmdResps) -> Result<ChangeSolEnumCmdRIds, BackrefRenderError> {
        Ok(match self {
            // Solar system
            Self::ChangeSol(cmd) => ChangeSolEnumCmdRIds::ChangeSol(cmd.basic),
            // Fleet
            Self::CreateFleet(cmd) => ChangeSolEnumCmdRIds::CreateFleet(cmd.basic.render(resps)?),
            Self::ChangeFleet(cmd) => ChangeSolEnumCmdRIds::ChangeFleet(cmd.basic.render(resps)?),
            Self::RemoveFleet(cmd) => ChangeSolEnumCmdRIds::RemoveFleet(cmd.basic.render(resps)?),
            // Fit
            Self::ChangeFit(cmd) => ChangeSolEnumCmdRIds::ChangeFit(cmd.basic.render(resps)?),
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
            Self::ChangeFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
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
    #[error("failed to change fit: {0}")]
    FitChangeFailed(#[from] GetFitChangeFitError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - solar system
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct SolChangeSolCmd {
    basic: CmdSolChangeFCtx,
}
impl SolChangeSolCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_sec_zone(mut self, sec_zone: rc::SecZone) -> Self {
        self.basic.sec_zone = Some(sec_zone);
        self
    }
    pub fn with_default_incoming_dps(mut self, incoming_dps: rc::DpsProfile) -> Self {
        self.basic.default_incoming_dps = Some(incoming_dps);
        self
    }
    pub fn with_default_spool(mut self, spool: rc::Spool) -> Self {
        self.basic.default_spool = Some(spool);
        self
    }
    pub fn with_default_npc_prop(mut self, npc_prop: rc::NpcProp) -> Self {
        self.basic.default_npc_prop = Some(npc_prop);
        self
    }
    pub fn with_default_optional_reloads(mut self, optional_reload: rc::OptionalReload) -> Self {
        self.basic.default_optional_reloads = Some(optional_reload);
        self
    }
    pub fn with_default_rearm_minions(mut self, rearm_minion: rc::RearmMinion) -> Self {
        self.basic.default_rearm_minions = Some(rearm_minion);
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
    basic: CmdFleetCreateFCtxBIds,
}
impl SolCreateFleetCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit_ids(mut self, fit_ids: impl ExactSizeIterator<Item = FitIdBackref>) -> Self {
        self.basic.fit_ids.clear();
        self.basic.fit_ids.extend(fit_ids);
        self
    }
}
impl From<SolCreateFleetCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolCreateFleetCmd) -> Self {
        Self::CreateFleet(sub_cmd)
    }
}

pub struct SolChangeFleetCmd {
    basic: CmdFleetChangeFCtxBIds,
}
impl SolChangeFleetCmd {
    pub fn new(fleet_id: FleetIdBackref) -> Self {
        Self {
            basic: CmdFleetChangeFCtxBIds {
                fleet_id,
                ictx_cmd: CmdFleetChangeICtxBIds::default(),
            },
        }
    }
    pub fn with_add_fit_ids(mut self, add_fit_ids: impl ExactSizeIterator<Item = FitIdBackref>) -> Self {
        self.basic.ictx_cmd.add_fit_ids.clear();
        self.basic.ictx_cmd.add_fit_ids.extend(add_fit_ids);
        self
    }
    pub fn with_rm_fit_ids(mut self, rm_fit_ids: impl ExactSizeIterator<Item = FitIdBackref>) -> Self {
        self.basic.ictx_cmd.rm_fit_ids.clear();
        self.basic.ictx_cmd.rm_fit_ids.extend(rm_fit_ids);
        self
    }
}
impl From<SolChangeFleetCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeFleetCmd) -> Self {
        Self::ChangeFleet(sub_cmd)
    }
}

pub struct SolRemoveFleetCmd {
    basic: CmdFleetRemoveFCtxBIds,
}
impl SolRemoveFleetCmd {
    pub fn new(fleet_id: FleetIdBackref) -> Self {
        Self {
            basic: CmdFleetRemoveFCtxBIds {
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
pub struct SolChangeFitCmd {
    basic: CmdFitChangeFCtxBIds,
}
impl SolChangeFitCmd {
    pub fn new(fit_id: FitIdBackref) -> Self {
        Self {
            basic: CmdFitChangeFCtxBIds {
                fit_id,
                ictx_cmd: CmdFitChangeICtxBIds::default(),
            },
        }
    }
    pub fn with_fleet_id(mut self, fleet_id: Option<FleetIdBackref>) -> Self {
        self.basic.ictx_cmd.fleet_id = fleet_id.into();
        self
    }
    pub fn with_sec_status(mut self, sec_status: rc::FitSecStatus) -> Self {
        self.basic.ictx_cmd.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: Option<rc::DpsProfile>) -> Self {
        self.basic.ictx_cmd.shared.rah_incoming_dps = rah_incoming_dps.into();
        self
    }
}
impl From<SolChangeFitCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeFitCmd) -> Self {
        Self::ChangeFit(sub_cmd)
    }
}
