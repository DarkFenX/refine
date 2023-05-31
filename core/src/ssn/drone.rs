use crate::{
    consts::State,
    defs::{ReeId, ReeInt},
    ssi,
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
impl From<&ssi::Drone> for DroneInfo {
    fn from(d: &ssi::Drone) -> Self {
        DroneInfo::new(d.id, d.fit_id, d.type_id, d.state)
    }
}
