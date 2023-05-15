use crate::{
    consts::State,
    defs::{ReeId, ReeInt},
    ss::item::Drone,
};

pub struct DroneInfo {
    pub item_id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub state: State,
}
impl DroneInfo {
    fn new(item_id: ReeId, fit_id: ReeId, type_id: ReeInt, state: State) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            state,
        }
    }
}
impl From<&Drone> for DroneInfo {
    fn from(d: &Drone) -> Self {
        DroneInfo::new(d.item_id, d.fit_id, d.type_id, d.state)
    }
}
