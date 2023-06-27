use crate::{
    consts::State,
    defs::{ReeId, ReeInt},
    ss::item::SsDrone,
};

pub struct SsDroneInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub a_item_id: ReeInt,
    pub state: State,
}
impl SsDroneInfo {
    fn new(id: ReeId, fit_id: ReeId, a_item_id: ReeInt, state: State) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
        }
    }
}
impl From<&SsDrone> for SsDroneInfo {
    fn from(ss_drone: &SsDrone) -> Self {
        SsDroneInfo::new(ss_drone.id, ss_drone.fit_id, ss_drone.a_item_id, ss_drone.state)
    }
}
