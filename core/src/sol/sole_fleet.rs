use itertools::Itertools;

use crate::{
    defs::SolFleetId,
    err::{AddFleetError, GetFleetInfoError, RemoveFleetError},
    sol::{fleet_info::SolFleetInfo, SolView, SolarSystem},
};

impl SolarSystem {
    pub fn get_fleet(&self, fleet_id: &SolFleetId) -> Result<SolFleetInfo, GetFleetInfoError> {
        let fleet = self.fleets.get_fleet(fleet_id)?;
        Ok(SolFleetInfo::from(fleet))
    }
    pub fn get_fleets(&self) -> Vec<SolFleetInfo> {
        self.fleets.iter_fleets().map(|v| SolFleetInfo::from(v)).collect()
    }
    pub fn add_fleet(&mut self) -> Result<SolFleetInfo, AddFleetError> {
        let fleet_id = self.fleets.add_fleet()?;
        let fleet = self.fleets.get_fleet(&fleet_id).unwrap();
        Ok(SolFleetInfo::from(fleet))
    }
    pub fn remove_fleet(&mut self, fleet_id: &SolFleetId) -> Result<(), RemoveFleetError> {
        let fleet = self.fleets.get_fleet(fleet_id)?;
        let fit_ids = fleet.iter_fits().map(|v| *v).collect_vec();
        for fit_id in fit_ids.iter() {
            self.svcs.remove_fit_from_fleet(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                fleet,
                fit_id,
            );
            let fit = self.fits.get_fit_mut(fit_id).unwrap();
            fit.fleet = None;
        }
        self.fleets.remove_fleet(fleet_id).unwrap();
        Ok(())
    }
}
