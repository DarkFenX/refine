use crate::{item::IntItemBase, Error, ErrorKind, FitChild, Result, Ship, SolarSystem};

pub struct Fit {
    sol_sys: Option<SolarSystem>,
    ship: Option<Ship>,
}
impl Fit {
    pub fn new(sol_sys: Option<SolarSystem>) -> Fit {
        Fit { sol_sys, ship: None }
    }
    pub(crate) fn get_sol_sys(&self) -> Option<&SolarSystem> {
        self.sol_sys.as_ref()
    }
    pub fn get_ship(&self) -> Option<&Ship> {
        self.ship.as_ref()
    }
    pub fn set_ship(&mut self, ship: Option<Ship>) -> Result<()> {
        match &ship {
            Some(s) => {
                if s.get_fit().is_some() {
                    return Err(Error::new(ErrorKind::AlreadyHasParent, "ship already has parent"));
                }
            }
            _ => (),
        }
        // TODO: handle removal of old ship
        self.ship = ship;
        self.ship.as_mut().map(|v| v.load_item());
        // TODO: handle addition of new ship
        Ok(())
    }
}
