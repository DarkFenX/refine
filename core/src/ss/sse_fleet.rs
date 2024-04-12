use crate::{defs::SsFleetId, util::Result};

use super::{fleet_info::SsFleetInfo, SolarSystem};

impl SolarSystem {
    pub fn add_fleet(&mut self) -> Result<SsFleetInfo> {
        let fleet_id = self.fleets.add_fleet()?;
        self.get_fleet(&fleet_id)
    }
    pub fn get_fleet(&self, fleet_id: &SsFleetId) -> Result<SsFleetInfo> {
        let fleet = self.fleets.get_fleet(fleet_id)?;
        Ok(fleet.into())
    }
    pub fn get_fleets(&self) -> Vec<SsFleetInfo> {
        self.fleets.iter_fleets().map(|v| v.into()).collect()
    }
    pub fn remove_fleet(&mut self, fleet_id: &SsFleetId) -> Result<()> {
        self.fleets.remove_fleet(fleet_id)?;
        Ok(())
    }
}
