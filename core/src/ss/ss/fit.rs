use crate::{
    defs::ReeId,
    ss::SolarSystem,
    util::{Error, ErrorKind, Result},
};

impl SolarSystem {
    // Public
    pub fn add_fit(&mut self) -> Result<ReeId> {
        let fit_id = self.alloc_fit_id()?;
        self.fits.insert(fit_id);
        Ok(fit_id)
    }
    pub fn remove_fit(&mut self, fit_id: &ReeId) -> Result<()> {
        self.items.drain_filter(|_, v| v.get_fit_id() == Some(*fit_id));
        match self.fits.remove(fit_id) {
            true => Ok(()),
            false => Err(Error::new(ErrorKind::FitNotFound, "fit not found")),
        }
    }
    pub fn get_fit_ids(&self) -> Vec<ReeId> {
        self.fits.iter().map(|v| *v).collect()
    }
    // Non-public
    fn alloc_fit_id(&mut self) -> Result<ReeId> {
        let start = self.fit_cnt;
        while self.fits.contains(&self.fit_cnt.0) {
            self.fit_cnt += 1;
            if start == self.fit_cnt {
                return Err(Error::new(ErrorKind::IdAllocFailed, "failed to allocate fit ID"));
            }
        }
        Ok(self.fit_cnt.0)
    }
    pub(in crate::ss) fn check_fit(&self, fit_id: &ReeId) -> Result<()> {
        match self.fits.contains(&fit_id) {
            true => Ok(()),
            false => Err(Error::new(ErrorKind::FitNotFound, "fit not found")),
        }
    }
}
