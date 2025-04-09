use crate::{
    err::basic::{FitFoundError, FitHasItemKindError},
    sol::{FitId, FitKey, SolarSystem, uad::item::Stance},
    util::Named,
};

impl SolarSystem {
    pub fn remove_fit_stance(&mut self, fit_id: &FitId) -> Result<(), RemoveFitStanceError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.remove_fit_stance_internal(fit_key)?)
    }
    pub(in crate::sol) fn remove_fit_stance_internal(&mut self, fit_key: FitKey) -> Result<(), FitHasItemKindError> {
        let fit = self.uad.fits.get(fit_key);
        let item_key = match fit.stance {
            Some(item_key) => item_key,
            None => {
                return Err(FitHasItemKindError {
                    fit_id: fit.id,
                    item_kind: Stance::get_name(),
                });
            }
        };
        self.remove_stance_internal(item_key).unwrap();
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveFitStanceError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
    #[error("{0}")]
    FitHasNoStance(#[from] FitHasItemKindError),
}
