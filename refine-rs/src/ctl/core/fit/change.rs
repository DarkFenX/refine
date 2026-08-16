use crate::{CtlCmdResps, DpsProfile, FitSecStatus, FleetId, FleetIdBackref, TriStateField, err::BackrefRenderError};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct FitChangeCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_id: TriStateField<FleetId> = TriStateField::Absent,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: CmdFitChangeShared = CmdFitChangeShared { .. },
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct FitChangeCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet_id: TriStateField<FleetIdBackref> = TriStateField::Absent,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: CmdFitChangeShared = CmdFitChangeShared { .. },
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
struct CmdFitChangeShared {
    sec_status: Option<FitSecStatus> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    rah_incoming_dps: TriStateField<DpsProfile> = TriStateField::Absent,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitChangeCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fleet_id(mut self, fleet_id: Option<FleetId>) -> Self {
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

impl FitChangeCmdBr {
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
impl FitChangeCmdBr {
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitChangeCmd {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> Result<(), FitChangeError> {
        match self.fleet_id {
            TriStateField::Value(fleet_id) => core_fit.set_fleet(&fleet_id)?,
            TriStateField::None => match core_fit.unset_fleet() {
                Ok(..) => (),
                // We are fine if fleet was not set
                Err(rc::err::UnsetFitFleetError::FitHasNoFleet(..)) => (),
            },
            TriStateField::Absent => (),
        }
        if let Some(sec_status) = self.shared.sec_status {
            core_fit.set_sec_status(sec_status);
        }
        match self.shared.rah_incoming_dps {
            TriStateField::Value(rah_incoming_dps) => core_fit.set_rah_incoming_dps(rah_incoming_dps),
            TriStateField::None => match core_fit.remove_rah_incoming_dps() {
                Ok(..) => (),
                // We are fine if profile was not set
                Err(rc::err::RemoveFitRahIncomingDpsError::DpsProfileNotSet(..)) => (),
            },
            TriStateField::Absent => (),
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitChangeError {
    #[error("failed to set fleet")]
    FleetSet(#[from] rc::err::SetFitFleetError),
}
