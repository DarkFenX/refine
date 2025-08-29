use crate::{
    err::basic::{FitHasCharacterError, FitHasShipError},
    sol::api::{CharacterMut, FitMut, ShipMut},
};

impl<'a> FitMut<'a> {
    pub(super) fn get_character_for_stats(&mut self) -> Result<CharacterMut<'_>, FitHasCharacterError> {
        let char_key = match self.sol.u_data.fits.get(self.key).character {
            Some(char_key) => char_key,
            None => {
                return Err(FitHasCharacterError {
                    fit_id: self.sol.u_data.fits.id_by_key(self.key),
                });
            }
        };
        Ok(CharacterMut::new(self.sol, char_key))
    }
    pub(super) fn get_ship_for_stats(&mut self) -> Result<ShipMut<'_>, FitHasShipError> {
        let ship_key = match self.sol.u_data.fits.get(self.key).ship {
            Some(ship_key) => ship_key,
            None => {
                return Err(FitHasShipError {
                    fit_id: self.sol.u_data.fits.id_by_key(self.key),
                });
            }
        };
        Ok(ShipMut::new(self.sol, ship_key))
    }
}
