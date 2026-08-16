use super::shared::CmdFitChangeShared;
use crate::{
    CtlCmdResps, DpsProfile, FitChangeCmd, FitSecStatus, FleetIdBackref, TriStateField, err::BackrefRenderError,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct FitChangeCmdBackref {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_id: TriStateField<FleetIdBackref> = TriStateField::Absent,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: CmdFitChangeShared = CmdFitChangeShared { .. },
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitChangeCmdBackref {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_id(mut self, fleet_id: Option<FleetIdBackref>) -> Self {
        self.fleet_id = fleet_id.into();
        self
    }
    pub fn with_sec_status(mut self, sec_status: FitSecStatus) -> Self {
        self.shared.sec_status = Some(sec_status);
        self
    }
    pub fn with_rah_incoming_dps(mut self, rah_incoming_dps: Option<DpsProfile>) -> Self {
        self.shared.rah_incoming_dps = rah_incoming_dps.into();
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitChangeCmdBackref {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<FitChangeCmd, BackrefRenderError> {
        Ok(FitChangeCmd {
            shared: self.shared,
            fleet_id: match self.fleet_id {
                TriStateField::Value(fleet_id) => TriStateField::Value(resps.render_fleet_id(fleet_id)?),
                TriStateField::None => TriStateField::None,
                TriStateField::Absent => TriStateField::Absent,
            },
        })
    }
}
