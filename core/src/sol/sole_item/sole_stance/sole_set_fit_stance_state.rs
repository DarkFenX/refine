use crate::{
    defs::SolFitId,
    err::basic::{FitFoundError, FitHasItemKindError},
    sol::{SolarSystem, uad::item::SolStance},
    util::Named,
};

impl SolarSystem {
    pub fn set_fit_stance_state(&mut self, fit_id: &SolFitId, state: bool) -> Result<(), SetFitStanceStateError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let item_id = match fit.stance {
            Some(item_id) => item_id,
            None => return Err(FitHasItemKindError::new(*fit_id, SolStance::get_name()).into()),
        };
        let stance = self.uad.items.get_item_mut(&item_id).unwrap().get_stance_mut().unwrap();
        let old_state = stance.get_state();
        stance.set_stance_state(state);
        let new_state = stance.get_state();
        self.change_item_id_state_in_svc(&item_id, old_state, new_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetFitStanceStateError {
    FitNotFound(FitFoundError),
    FitHasNoStance(FitHasItemKindError),
}
impl std::error::Error for SetFitStanceStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::FitHasNoStance(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitStanceStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::FitHasNoStance(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for SetFitStanceStateError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<FitHasItemKindError> for SetFitStanceStateError {
    fn from(error: FitHasItemKindError) -> Self {
        Self::FitHasNoStance(error)
    }
}
