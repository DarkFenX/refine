use crate::cmd::{
    inner::{
        CmdFitChangeICtxBIds, CmdFitChangeICtxRIds, CmdItemRemoveFCtxBIds, CmdItemRemoveFCtxRIds, CmdItemRemoveICtx,
        FitChangeFitError, GetItemRemoveItemError,
    },
    shared::{BackrefRenderError, CmdResp, CmdResps, FleetIdBackref, ItemIdBackref},
};

pub enum ChangeFitEnumCmd {
    // Fit
    ChangeFit(FitChangeFitCmd),
    // Item
    RemoveItem(FitRemoveItemCmd),
}

pub(crate) enum ChangeFitEnumCmdRIds {
    // Fit
    ChangeFit(CmdFitChangeICtxRIds),
    // Item
    RemoveItem(CmdItemRemoveFCtxRIds),
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
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeFitEnumError {
    #[error("failed to change fleet: {0}")]
    FitChangeFailed(#[from] FitChangeFitError),
    #[error("failed to remove item: {0}")]
    ItemRemoveFailed(#[from] GetItemRemoveItemError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - fit
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct FitChangeFitCmd {
    inner: CmdFitChangeICtxBIds,
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
    inner: CmdItemRemoveFCtxBIds,
}
impl FitRemoveItemCmd {
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
impl From<FitRemoveItemCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitRemoveItemCmd) -> Self {
        Self::RemoveItem(sub_cmd)
    }
}
