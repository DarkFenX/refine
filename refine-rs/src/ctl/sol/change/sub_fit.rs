use crate::{DpsProfile, FitIdBackref, FitSecStatus, FleetIdBackref, SolCtlCmd, ctl::core::ICmdFitChangeFCtxBIds};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolChangeFitCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
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
    pub fn with_sec_status(mut self, sec_status: FitSecStatus) -> Self {
        self.inner.ictx_cmd.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: Option<DpsProfile>) -> Self {
        self.inner.ictx_cmd.shared.rah_incoming_dps = rah_incoming_dps.into();
        self
    }
}
impl From<SolChangeFitCmd> for SolCtlCmd {
    fn from(sub_cmd: SolChangeFitCmd) -> Self {
        Self::ChangeFit(sub_cmd)
    }
}
