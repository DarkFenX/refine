use crate::sol::{SolarSystem, info::DroneInfo, uad::item::Drone};

impl SolarSystem {
    pub(in crate::sol) fn make_drone_info(&self, drone: &Drone) -> DroneInfo {
        DroneInfo::from_drone_with_source(&self.uad.src, drone)
    }
}
