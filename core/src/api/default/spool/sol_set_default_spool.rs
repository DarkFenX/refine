use crate::{misc::Spool, sol::SolarSystem};

impl SolarSystem {
    pub fn set_default_spool(&mut self, spool: Spool) {
        self.u_data.default_spool = spool;
    }
}
