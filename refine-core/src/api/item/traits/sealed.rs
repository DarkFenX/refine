use crate::{sol::SolarSystem, ud::UItemId};

pub(in crate::api) trait ItemSealed: Sized {
    fn get_uid(&self) -> UItemId;
    fn get_sol_ref(&self) -> &SolarSystem;
}
