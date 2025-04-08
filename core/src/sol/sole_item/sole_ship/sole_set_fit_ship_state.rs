use crate::{
    err::basic::{FitFoundError, FitHasItemKindError},
    sol::{FitId, FitKey, SolarSystem, uad::item::Ship},
    util::Named,
};

impl SolarSystem {
    pub fn set_fit_ship_state(&mut self, fit_id: &FitId, state: bool) -> Result<(), SetFitShipStateError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.set_fit_ship_state_internal(fit_key, state)?)
    }
    pub(in crate::sol) fn set_fit_ship_state_internal(
        &mut self,
        fit_key: FitKey,
        state: bool,
    ) -> Result<(), FitHasItemKindError> {
        let fit = self.uad.fits.get(fit_key);
        let item_key = match fit.ship {
            Some(item_key) => item_key,
            None => {
                return Err(FitHasItemKindError {
                    fit_id: fit.id,
                    item_kind: Ship::get_name(),
                }
                .into());
            }
        };
        let ship = self.uad.items.get_mut(item_key).get_ship_mut().unwrap();
        let old_a_state = ship.get_a_state();
        ship.set_ship_state(state);
        let new_a_state = ship.get_a_state();
        self.change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
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
