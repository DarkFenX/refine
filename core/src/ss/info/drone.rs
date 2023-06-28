use crate::{
    consts::State,
    defs::{EItemId, SsFitId, SsItemId},
    ss::item::SsDrone,
};

pub struct SsDroneInfo {
    pub id: SsItemId,
    pub fit_id: SsFitId,
    pub a_item_id: EItemId,
    pub state: State,
}
impl SsDroneInfo {
    fn new(id: SsItemId, fit_id: SsFitId, a_item_id: EItemId, state: State) -> Self {
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
