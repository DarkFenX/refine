use crate::sol::SolarSystem;

impl SolarSystem {
    pub fn get_default_optional_reloads(&self) -> bool {
        self.u_data.default_optional_reloads
    }
}
