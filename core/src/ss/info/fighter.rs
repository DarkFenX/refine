use crate::{
    consts::State,
    defs::{Amount, ItemId, SsFitId, SsItemId},
    ss::item::SsFighter,
};

pub struct SsFighterInfo {
    pub id: SsItemId,
    pub fit_id: SsFitId,
    pub a_item_id: ItemId,
    pub state: State,
    pub amt_override: Option<Amount>,
}
impl SsFighterInfo {
    fn new(id: SsItemId, fit_id: SsFitId, a_item_id: ItemId, state: State, amt_override: Option<Amount>) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
            amt_override,
        }
    }
}
impl From<&SsFighter> for SsFighterInfo {
    fn from(ss_fighter: &SsFighter) -> Self {
        SsFighterInfo::new(
            ss_fighter.id,
            ss_fighter.fit_id,
            ss_fighter.a_item_id,
            ss_fighter.state,
            ss_fighter.amt_override,
        )
    }
}
