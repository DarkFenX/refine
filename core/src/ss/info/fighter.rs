use crate::{
    consts::State,
    defs::{ReeId, ReeInt},
    ss::item::Fighter,
};

pub struct FighterInfo {
    pub item_id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub state: State,
    pub amt_override: Option<ReeInt>,
}
impl FighterInfo {
    fn new(item_id: ReeId, fit_id: ReeId, type_id: ReeInt, state: State, amt_override: Option<ReeInt>) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            state,
            amt_override,
        }
    }
}
impl From<&Fighter> for FighterInfo {
    fn from(f: &Fighter) -> Self {
        FighterInfo::new(f.item_id, f.fit_id, f.type_id, f.state, f.amt_override)
    }
}
