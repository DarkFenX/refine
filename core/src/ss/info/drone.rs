use crate::{
    consts::State,
    defs::{ReeId, ReeInt},
    ss::item::Drone,
};

pub struct DroneInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub state: State,
}
impl DroneInfo {
    fn new(id: ReeId, fit_id: ReeId, type_id: ReeInt, state: State) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            state,
        }
    }
}
impl From<&Drone> for DroneInfo {
    fn from(d: &Drone) -> Self {
        DroneInfo::new(d.id, d.fit_id, d.type_id, d.state)
    }
}
