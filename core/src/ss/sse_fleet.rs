use crate::{defs::SsFleetId, util::Result};

use super::{fleet_info::SsFleetInfo, SolarSystem};

impl SolarSystem {
    pub fn get_fleet_ids(&self) -> Vec<SsFleetId> {
        self.fleets.iter_fleet_ids().map(|v| *v).collect()
    }
    pub fn get_fleet_info(&self, fleet_id: &SsFleetId) -> Result<SsFleetInfo> {
        let fleet = self.fleets.get_fleet(fleet_id)?;
        Ok(fleet.into())
    }
    pub fn get_fleet_infos(&self) -> Vec<SsFleetInfo> {
        self.fleets.iter_fleets().map(|v| v.into()).collect()
    }
    pub fn add_fleet(&mut self) -> Result<SsFleetId> {
        let fleet_id = self.fleets.add_fleet()?;
        Ok(fleet_id)
    }
    pub fn remove_fleet(&mut self, fleet_id: &SsFleetId) -> Result<()> {
        self.fleets.remove_fleet(fleet_id)?;
        Ok(())
    }
}
