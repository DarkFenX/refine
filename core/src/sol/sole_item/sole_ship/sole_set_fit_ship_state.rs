use crate::{
    defs::SolFitId,
    err::basic::{FitFoundError, FitHasItemKindError},
    sol::{item::SolShip, SolarSystem},
    util::Named,
};

impl SolarSystem {
    pub fn set_fit_ship_state(&mut self, fit_id: &SolFitId, state: bool) -> Result<(), SetFitShipStateError> {
        let fit = self.fits.get_fit(fit_id)?;
        let item_id = match fit.ship {
            Some(item_id) => item_id,
            None => return Err(FitHasItemKindError::new(*fit_id, SolShip::get_name()).into()),
        };
        let ship = self.items.get_item_mut(&item_id).unwrap().get_ship_mut().unwrap();
        let old_state = ship.state;
        ship.set_bool_state(state);
        let new_state = ship.state;
        self.change_item_id_state_in_svcs(&item_id, old_state, new_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetFitShipStateError {
    FitNotFound(FitFoundError),
    FitHasNoShip(FitHasItemKindError),
}
impl std::error::Error for SetFitShipStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::FitHasNoShip(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitShipStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::FitHasNoShip(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for SetFitShipStateError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<FitHasItemKindError> for SetFitShipStateError {
    fn from(error: FitHasItemKindError) -> Self {
        Self::FitHasNoShip(error)
    }
}
