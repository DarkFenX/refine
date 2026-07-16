use crate::{
    ChangeSolEnumCmd, FitIdBackref, FleetIdBackref,
    cmd::inner::{ICmdFitAddFCtxBIds, ICmdFitChangeFCtxBIds, ICmdFitRemoveFCtxBIds},
};

#[derive(Default)]
pub struct SolAddFitCmd {
    pub(super) inner: ICmdFitAddFCtxBIds = ICmdFitAddFCtxBIds { .. },
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
    pub(super) inner: ICmdFitChangeFCtxBIds,
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
    pub(super) inner: ICmdFitRemoveFCtxBIds,
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
