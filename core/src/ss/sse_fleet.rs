use itertools::Itertools;

use crate::{defs::SsFleetId, util::Result};

use super::{fleet_info::SsFleetInfo, SolarSystem};

impl SolarSystem {
    pub fn get_fleet(&self, fleet_id: &SsFleetId) -> Result<SsFleetInfo> {
        let fleet = self.fleets.get_fleet(fleet_id)?;
        Ok(fleet.into())
    }
    pub fn get_fleets(&self) -> Vec<SsFleetInfo> {
        self.fleets.iter_fleets().map(|v| v.into()).collect()
    }
    pub fn add_fleet(&mut self) -> Result<SsFleetInfo> {
        let fleet_id = self.fleets.add_fleet()?;
        self.get_fleet(&fleet_id)
    }
    pub fn remove_fleet(&mut self, fleet_id: &SsFleetId) -> Result<()> {
        let fit_ids = self.fleets.get_fleet(fleet_id)?.iter_fits().map(|v| *v).collect_vec();
        for fit_id in fit_ids.iter() {
            self.set_fit_fleet(fit_id, None).unwrap();
        }
        self.fleets.remove_fleet(fleet_id)?;
        Ok(())
    }
}
