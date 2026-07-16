use crate::{ChangeFitEnumCmd, DpsProfile, FitSecStatus, FleetIdBackref, cmd::inner::ICmdFitChangeICtxBIds};

#[derive(Default)]
pub struct FitChangeFitCmd {
    pub(super) inner: ICmdFitChangeICtxBIds = ICmdFitChangeICtxBIds { .. },
}
impl FitChangeFitCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_id(mut self, fleet_id: Option<FleetIdBackref>) -> Self {
        self.inner.fleet_id = fleet_id.into();
        self
    }
    pub fn with_sec_status(mut self, sec_status: FitSecStatus) -> Self {
        self.inner.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: Option<DpsProfile>) -> Self {
        self.inner.shared.rah_incoming_dps = rah_incoming_dps.into();
        self
    }
}
impl From<FitChangeFitCmd> for ChangeFitEnumCmd {
    fn from(sub_cmd: FitChangeFitCmd) -> Self {
        Self::ChangeFit(sub_cmd)
    }
}
