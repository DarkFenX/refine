use crate::{
    defs::{SolFitId, SolFleetId},
    err::{AddFitError, FitFleetAssignedError, GetFitInfoError, RemoveFitError, SetFitFleetError, UnsetFitFleetError},
    sol::SolView,
};

use super::{fit_info::SolFitInfo, SolarSystem};

impl SolarSystem {
    pub fn get_fit(&self, fit_id: &SolFitId) -> Result<SolFitInfo, GetFitInfoError> {
        let fit = self.fits.get_fit(fit_id)?;
        Ok(SolFitInfo::from(fit))
    }
    pub fn get_fits(&self) -> Vec<SolFitInfo> {
        self.fits.iter_fits().map(|v| v.into()).collect()
    }
    pub fn add_fit(&mut self) -> Result<SolFitInfo, AddFitError> {
        let fit_id = self.fits.add_fit()?;
        self.svcs.add_fit(&fit_id);
        let fit = self.fits.get_fit(&fit_id).unwrap();
        Ok(SolFitInfo::from(fit))
    }
    pub fn set_fit_fleet(&mut self, fit_id: &SolFitId, fleet_id: SolFleetId) -> Result<(), SetFitFleetError> {
        let fit = self.fits.get_fit(fit_id)?;
        self.fleets.get_fleet(&fleet_id)?;
        // Unassign from old fleet
        if let Some(old_fleet_id) = fit.fleet {
            let old_fleet = self.fleets.get_fleet(&old_fleet_id).unwrap();
            self.svcs.remove_fit_from_fleet(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                old_fleet,
                fit_id,
            );
            let old_fleet = self.fleets.get_fleet_mut(&fleet_id).unwrap();
            old_fleet.remove_fit(fit_id);
        }
        // Assign new fleet
        let fit = self.fits.get_fit_mut(fit_id).unwrap();
        fit.fleet = Some(fleet_id);
        let fleet = self.fleets.get_fleet_mut(&fleet_id).unwrap();
        fleet.add_fit(*fit_id);
        self.svcs.add_fit_to_fleet(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            self.fleets.get_fleet(&fleet_id).unwrap(),
            fit_id,
        );
        Ok(())
    }
    pub fn unset_fit_fleet(&mut self, fit_id: &SolFitId) -> Result<(), UnsetFitFleetError> {
        let fit = self.fits.get_fit(fit_id)?;
        let fleet_id = match fit.fleet {
            Some(fleet_id) => fleet_id,
            None => return Err(FitFleetAssignedError::new(*fit_id).into()),
        };
        let fleet = self.fleets.get_fleet(&fleet_id).unwrap();
        self.svcs.remove_fit_from_fleet(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            fleet,
            fit_id,
        );
        let fleet = self.fleets.get_fleet_mut(&fleet_id).unwrap();
        fleet.remove_fit(fit_id);
        let fit = self.fits.get_fit_mut(fit_id).unwrap();
        fit.fleet = None;
        Ok(())
    }
    pub fn remove_fit(&mut self, fit_id: &SolFitId) -> Result<(), RemoveFitError> {
        let fit = self.fits.get_fit(fit_id)?;
        for item_id in fit.all_items().iter() {
            self.remove_item(item_id).unwrap();
        }
        self.svcs.remove_fit(&fit_id);
        let fit = self.fits.remove_fit(fit_id).unwrap();
        if let Some(fleet_id) = fit.fleet {
            let fleet = self.fleets.get_fleet_mut(&fleet_id).unwrap();
            fleet.remove_fit(fit_id);
        }
        Ok(())
    }
}
