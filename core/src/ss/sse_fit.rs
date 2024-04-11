use crate::{defs::SsFitId, util::Result};

use super::{fit_info::SsFitInfo, SolarSystem};

impl SolarSystem {
    pub fn get_fit_ids(&self) -> Vec<SsFitId> {
        self.fits.iter_fit_ids().map(|v| *v).collect()
    }
    pub fn get_fit_info(&self, fit_id: &SsFitId) -> Result<SsFitInfo> {
        let fit = self.fits.get_fit(fit_id)?;
        Ok(fit.into())
    }
    pub fn get_fit_infos(&self) -> Vec<SsFitInfo> {
        self.fits.iter_fits().map(|v| v.into()).collect()
    }
    pub fn add_fit(&mut self) -> Result<SsFitId> {
        let fit_id = self.fits.add_fit()?;
        self.svcs.add_fit(&fit_id);
        Ok(fit_id)
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
