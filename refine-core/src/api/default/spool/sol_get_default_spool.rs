use crate::{SolarSystem, Spool};

impl SolarSystem {
    pub fn get_default_spool(&self) -> Spool {
        self.u_data.default_spool
    }
}
