use crate::{
    consts::State,
    defs::{ReeId, ReeInt},
    ssi,
};

pub struct SsFighterInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub a_item_id: ReeInt,
    pub state: State,
    pub amt_override: Option<ReeInt>,
}
impl SsFighterInfo {
    fn new(id: ReeId, fit_id: ReeId, a_item_id: ReeInt, state: State, amt_override: Option<ReeInt>) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
            amt_override,
        }
    }
}
impl From<&ssi::SsFighter> for SsFighterInfo {
    fn from(ss_fighter: &ssi::SsFighter) -> Self {
        SsFighterInfo::new(
            ss_fighter.id,
            ss_fighter.fit_id,
            ss_fighter.a_item_id,
            ss_fighter.state,
            ss_fighter.amt_override,
        )
    }
}
