use crate::{
    defs::{ReeId, ReeInt},
    ss::item::Ship,
};

pub struct ShipInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub enabled: bool,
}
impl ShipInfo {
    fn new(id: ReeId, fit_id: ReeId, type_id: ReeInt, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            enabled,
        }
    }
}
impl From<&Ship> for ShipInfo {
    fn from(s: &Ship) -> Self {
        ShipInfo::new(s.id, s.fit_id, s.type_id, s.get_bool_state())
    }
}
