use crate::{fleet::Fleet, sol::SolarSystem};

impl<'r, 's> SolarSystem<'r> {
    pub async fn create_fleet(&'s mut self) -> Fleet<'r, 's> {
        let fleet_id = self
            .get_inner()
            .core_sol
            .as_mut()
            .unwrap()
            .create_fleet()
            .get_fleet_id();
        Fleet::new(self, fleet_id)
    }
}
