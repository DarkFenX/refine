use crate::sol::{info::SolDroneInfo, uad::item::SolDrone, SolarSystem};

impl SolarSystem {
    pub(in crate::sol) fn make_drone_info(&self, drone: &SolDrone) -> SolDroneInfo {
        SolDroneInfo::from_drone_with_source(&self.uad.src, drone)
    }
}
