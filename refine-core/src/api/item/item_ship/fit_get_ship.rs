use crate::{Fit, Ship};

impl<'s> Fit<'s> {
    pub fn get_ship(&mut self) -> Option<Ship<'_>> {
        self.sol
            .u_data
            .fits
            .get(self.uid)
            .ship
            .map(|ship_uid| Ship::new(self.sol, ship_uid))
    }
}
