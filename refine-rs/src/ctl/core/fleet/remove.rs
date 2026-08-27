use crate::{CmdResps, FleetId, FleetIdBr, err::BrResolveError, shared::CmdResidue};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FleetRemoveCmd;

// Extra context commands
pub type FleetRemoveCmdCtxFleet = FleetRemoveCmdCtxFleetGen<FleetId>;
pub type FleetRemoveCmdCtxFleetBr = FleetRemoveCmdCtxFleetGen<FleetIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "L: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FleetRemoveCmdCtxFleetGen<L> {
    fleet_id: L,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FleetRemoveCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetRemoveCmd {
    pub fn new() -> Self {
        Self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetRemoveCmd {
    pub(in crate::ctl) fn into_ctx_fleet(self, fleet_id: FleetId) -> FleetRemoveCmdCtxFleet {
        FleetRemoveCmdCtxFleet { fleet_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_fleet_br(self, fleet_id: impl Into<FleetIdBr>) -> FleetRemoveCmdCtxFleetBr {
        FleetRemoveCmdCtxFleetBr {
            fleet_id: fleet_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetRemoveCmdCtxFleetBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<FleetRemoveCmdCtxFleet, BrResolveError> {
        Ok(FleetRemoveCmdCtxFleet {
            fleet_id: resps.resolve_fleet_id(self.fleet_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetRemoveCmd {
    fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutInfallible
    }
}
impl FleetRemoveCmdCtxFleet {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}
impl FleetRemoveCmdCtxFleetBr {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}

impl FleetRemoveCmd {
    pub(crate) fn execute(self, core_fleet: rc::FleetMut) {
        core_fleet.remove()
    }
}

impl FleetRemoveCmdCtxFleet {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<(), FleetGetFleetRemoveError> {
        let core_fleet = core_sol.get_fleet_mut(&self.fleet_id)?;
        self.core.execute(core_fleet);
        Ok(())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FleetGetFleetRemoveError {
    #[error(transparent)]
    FleetGet(#[from] rc::err::FleetGetError),
}
