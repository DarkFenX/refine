use super::shared::FitAddCmdShared;
use crate::{CtlCmdResps, DpsProfile, FitAddCmd, FitSecStatus, FleetIdBackref, err::BackrefRenderError};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct FitAddCmdBackref {
    fleet_id: Option<FleetIdBackref> = None,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FitAddCmdShared = FitAddCmdShared { .. },
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitAddCmdBackref {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_id(mut self, fleet_id: FleetIdBackref) -> Self {
        self.fleet_id = Some(fleet_id);
        self
    }
    pub fn with_sec_status(mut self, sec_status: FitSecStatus) -> Self {
        self.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: DpsProfile) -> Self {
        self.shared.rah_incoming_dps = Some(rah_incoming_dps);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitAddCmdBackref {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<FitAddCmd, BackrefRenderError> {
        Ok(FitAddCmd {
            shared: self.shared,
            fleet_id: match self.fleet_id {
                Some(fleet_id) => Some(resps.render_fleet_id(fleet_id)?),
                None => None,
            },
        })
    }
}
