use crate::{
    err::basic::{FitDpsProfileFoundError, FitFoundError},
    sol::{FitId, FitKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_fit_rah_incoming_dps(&mut self, fit_id: &FitId) -> Result<(), RemoveFitRahIncomingDpsError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.remove_fit_rah_incoming_dps_internal(fit_key)?)
    }
    pub(in crate::sol) fn remove_fit_rah_incoming_dps_internal(
        &mut self,
        fit_key: FitKey,
    ) -> Result<(), FitDpsProfileFoundError> {
        let fit = self.uad.fits.get_mut(fit_key);
        let old_dps_profile = fit.rah_incoming_dps.take();
        match old_dps_profile {
            Some(old_dps_profile) => {
                // Do not trigger anything in services if effectively RAH profile is not changed -
                // RAH sim uses default incoming dps if RAH profile is not set
                if self.uad.default_incoming_dps != old_dps_profile {
                    self.svc.default_incoming_dps_profile_changed(&self.uad);
                }
            }
            None => return Err(FitDpsProfileFoundError { fit_id: fit.id }),
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveFitRahIncomingDpsError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
    #[error("{0}")]
    DpsProfileNotSet(#[from] FitDpsProfileFoundError),
}
