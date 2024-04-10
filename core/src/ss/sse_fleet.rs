use crate::{defs::SsFleetId, util::Result};

use super::SolarSystem;

impl SolarSystem {
    pub fn add_fleet(&mut self) -> Result<SsFleetId> {
        let fleet_id = self.fleets.add_fleet()?;
        Ok(fleet_id)
    }
    pub fn remove_fleet(&mut self, fleet_id: &SsFleetId) -> Result<()> {
        self.fleets.remove_fleet(fleet_id)?;
        Ok(())
    }
    pub fn get_fleet_ids(&self) -> Vec<SsFleetId> {
        self.fleets.iter_fleet_ids().map(|v| *v).collect()
    }
}
