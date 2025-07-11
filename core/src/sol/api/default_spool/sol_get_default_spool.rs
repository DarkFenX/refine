use crate::{misc::Spool, sol::SolarSystem};

impl SolarSystem {
    pub fn get_default_spool(&self) -> Spool {
        self.uad.default_spool
    }
}
