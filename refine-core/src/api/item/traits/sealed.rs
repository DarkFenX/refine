use crate::{SolarSystem, ud::UItemId};

pub(in crate::api) trait ItemSealed: Sized {
    fn get_uid(&self) -> UItemId;
    fn get_sol(&self) -> &SolarSystem;
    fn get_sol_mut(&mut self) -> &mut SolarSystem;
}
