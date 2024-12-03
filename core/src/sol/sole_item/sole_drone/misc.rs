use crate::sol::{item::SolDrone, item_info::SolDroneInfo, SolarSystem};

impl SolarSystem {
    pub(in crate::sol) fn make_drone_info(&self, drone: &SolDrone) -> SolDroneInfo {
        SolDroneInfo::from_drone_with_source(&self.src, drone)
    }
}
