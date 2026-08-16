use crate::{CtlCmdResps, FitId, FitIdBackref, FleetId, FleetIdBackref, err::BackrefRenderError};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdFleetChangeFCtxBIds {
    pub(in crate::ctl) fleet_id: FleetIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdFleetChangeICtxBIds = ICmdFleetChangeICtxBIds { .. },
}
pub(crate) struct ICmdFleetChangeFCtxRIds {
    fleet_id: FleetId,
    ictx_cmd: ICmdFleetChangeICtxRIds,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdFleetChangeICtxBIds {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) add_fit_ids: Vec<FitIdBackref> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) rm_fit_ids: Vec<FitIdBackref> = Vec::new(),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdFleetChangeICtxRIds {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) add_fit_ids: Vec<FitId> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) rm_fit_ids: Vec<FitId> = Vec::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFleetChangeFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdFleetChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdFleetChangeFCtxRIds {
            fleet_id: resps.render_fleet_id(self.fleet_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl ICmdFleetChangeICtxBIds {
    fn render(self, resps: &CtlCmdResps) -> Result<ICmdFleetChangeICtxRIds, BackrefRenderError> {
        Ok(ICmdFleetChangeICtxRIds {
            add_fit_ids: resps.render_fit_ids(self.add_fit_ids)?,
            rm_fit_ids: resps.render_fit_ids(self.rm_fit_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFleetChangeFCtxRIds {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), GetFleetChangeFleetError> {
        let mut core_fleet = core_sol.get_fleet_mut(&self.fleet_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fleet)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFleetChangeFleetError {
    #[error(transparent)]
    FleetGet(#[from] rc::err::GetFleetError),
    #[error("failed to add fit")]
    FitAdd(#[source] rc::err::FleetAddFitError),
    #[error("failed to remove fit")]
    FitRemove(#[source] rc::err::FleetRemoveFitError),
}
impl From<FleetChangeFleetError> for GetFleetChangeFleetError {
    fn from(err: FleetChangeFleetError) -> Self {
        match err {
            FleetChangeFleetError::FitAdd(inner) => Self::FitAdd(inner),
            FleetChangeFleetError::FitRemove(inner) => Self::FitRemove(inner),
        }
    }
}

impl ICmdFleetChangeICtxRIds {
    pub(in crate::ctl) fn execute(self, core_fleet: &mut rc::FleetMut) -> Result<(), FleetChangeFleetError> {
        for fit_id in self.rm_fit_ids.iter() {
            core_fleet.remove_fit(fit_id)?;
        }
        for fit_id in self.add_fit_ids.iter() {
            core_fleet.add_fit(fit_id)?;
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FleetChangeFleetError {
    #[error("failed to add fit")]
    FitAdd(#[from] rc::err::FleetAddFitError),
    #[error("failed to remove fit")]
    FitRemove(#[from] rc::err::FleetRemoveFitError),
}
