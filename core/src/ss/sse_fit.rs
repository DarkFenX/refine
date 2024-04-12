use crate::{defs::SsFitId, util::Result};

use super::{fit_info::SsFitInfo, SolarSystem};

impl SolarSystem {
    pub fn add_fit(&mut self) -> Result<SsFitInfo> {
        let fit_id = self.fits.add_fit()?;
        self.svcs.add_fit(&fit_id);
        self.get_fit(&fit_id)
    }
    pub fn get_fit(&self, fit_id: &SsFitId) -> Result<SsFitInfo> {
        let fit = self.fits.get_fit(fit_id)?;
        Ok(fit.into())
    }
    pub fn get_fits(&self) -> Vec<SsFitInfo> {
        self.fits.iter_fits().map(|v| v.into()).collect()
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
