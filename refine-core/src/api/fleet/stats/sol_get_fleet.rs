use crate::{
    FleetId,
    err::FleetGetError,
    stats::{FleetStats, SolStats},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> SolStats<'s> {
    pub fn get_fleet(&mut self, fleet_id: &FleetId) -> Result<FleetStats<'_>, FleetGetError> {
        let fleet_uid = self.sol.internal_get_sol_fleet_uid(fleet_id)?;
        Ok(FleetStats::new(self.sol, fleet_uid))
    }
}
