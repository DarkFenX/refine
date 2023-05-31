use crate::{
    consts::State,
    defs::{ReeId, ReeInt},
    ssi,
};

pub struct FighterInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub state: State,
    pub amt_override: Option<ReeInt>,
}
impl FighterInfo {
    fn new(id: ReeId, fit_id: ReeId, type_id: ReeInt, state: State, amt_override: Option<ReeInt>) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            state,
            amt_override,
        }
    }
}
impl From<&ssi::Fighter> for FighterInfo {
    fn from(f: &ssi::Fighter) -> Self {
        FighterInfo::new(f.id, f.fit_id, f.type_id, f.state, f.amt_override)
    }
}
