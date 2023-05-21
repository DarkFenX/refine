use crate::{
    defs::{ReeId, ReeInt},
    ss::item::Charge,
};

pub struct ChargeInfo {
    pub item_id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub cont: ReeId,
    pub enabled: bool,
}
impl ChargeInfo {
    fn new(item_id: ReeId, fit_id: ReeId, type_id: ReeInt, cont: ReeId, enabled: bool) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            cont,
            enabled,
        }
    }
}
impl From<&Charge> for ChargeInfo {
    fn from(c: &Charge) -> Self {
        ChargeInfo::new(c.item_id, c.fit_id, c.type_id, c.container_id, c.get_bool_state())
    }
}
