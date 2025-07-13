use crate::{
    err::basic::FitHasShipError,
    sol::api::{FitMut, ShipMut},
};

impl<'a> FitMut<'a> {
    pub(super) fn get_ship_for_stats(&mut self) -> Result<ShipMut<'_>, FitHasShipError> {
        let ship_key = match self.sol.uad.fits.get(self.key).ship {
            Some(ship_key) => ship_key,
            None => {
                return Err(FitHasShipError {
                    fit_id: self.sol.uad.fits.id_by_key(self.key),
                });
            }
        };
        Ok(ShipMut::new(self.sol, ship_key))
    }
}
