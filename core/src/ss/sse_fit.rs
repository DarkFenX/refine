use crate::{
    defs::{SsFitId, SsFleetId},
    ss::SsView,
    util::Result,
};

use super::{fit_info::SsFitInfo, SolarSystem};

impl SolarSystem {
    pub fn get_fit(&self, fit_id: &SsFitId) -> Result<SsFitInfo> {
        let fit = self.fits.get_fit(fit_id)?;
        Ok(fit.into())
    }
    pub fn get_fits(&self) -> Vec<SsFitInfo> {
        self.fits.iter_fits().map(|v| v.into()).collect()
    }
    pub fn add_fit(&mut self) -> Result<SsFitInfo> {
        let fit_id = self.fits.add_fit()?;
        self.svcs.add_fit(&fit_id);
        self.get_fit(&fit_id)
    }
    pub fn set_fit_fleet(&mut self, fit_id: &SsFitId, fleet_id_opt: Option<SsFleetId>) -> Result<()> {
        let fit = self.fits.get_fit(fit_id)?;
        let old_fleet_id_opt = fit.fleet;
        // Unassign from old fleet
        if let Some(old_fleet_id) = old_fleet_id_opt {
            if let Ok(old_fleet) = self.fleets.get_fleet(&old_fleet_id) {
                self.svcs.remove_fit_from_fleet(
                    &SsView::new(&self.src, &self.fleets, &self.fits, &self.items),
                    old_fleet,
                    fit_id,
                );
            }

            if let Ok(old_fleet) = self.fleets.get_fleet_mut(&old_fleet_id) {
                old_fleet.remove_fit(fit_id);
            }
            let fit = self.fits.get_fit_mut(fit_id)?;
            fit.fleet = None;
        }
        if let Some(new_fleet_id) = fleet_id_opt {
            match self.fleets.get_fleet_mut(&new_fleet_id) {
                // Assign to new fleet
                Ok(new_fleet) => {
                    new_fleet.add_fit(*fit_id);
                    let fit = self.fits.get_fit_mut(fit_id)?;
                    fit.fleet = Some(new_fleet_id);
                    self.svcs.add_fit_to_fleet(
                        &SsView::new(&self.src, &self.fleets, &self.fits, &self.items),
                        self.fleets.get_fleet(&new_fleet_id).unwrap(),
                        fit_id,
                    );
                }
                // If assignment failed, revert to old fleet. Since we started with a clear state
                // before we tried to assign new fleet, do something only if there was an old fleet
                _ => {
                    if let Some(old_fleet_id) = old_fleet_id_opt {
                        let fit = self.fits.get_fit_mut(fit_id)?;
                        fit.fleet = Some(old_fleet_id);
                        if let Ok(old_fleet) = self.fleets.get_fleet_mut(&old_fleet_id) {
                            old_fleet.add_fit(old_fleet_id);
                            self.svcs.add_fit_to_fleet(
                                &SsView::new(&self.src, &self.fleets, &self.fits, &self.items),
                                self.fleets.get_fleet(&old_fleet_id).unwrap(),
                                fit_id,
                            );
                        }
                    }
                }
            }
        }
        Ok(())
    }
    pub fn remove_fit(&mut self, fit_id: &SsFitId) -> Result<()> {
        for item_id in self.fits.get_fit(fit_id)?.all_items().iter() {
            self.remove_item(item_id).unwrap();
        }
        self.svcs.remove_fit(&fit_id);
        self.fits.remove_fit(fit_id)?;
        Ok(())
    }
}
