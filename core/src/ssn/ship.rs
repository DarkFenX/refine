use crate::{
    defs::{ReeId, ReeInt},
    ssi,
};

pub struct SsShipInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub a_item_id: ReeInt,
    pub enabled: bool,
}
impl SsShipInfo {
    fn new(id: ReeId, fit_id: ReeId, a_item_id: ReeInt, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&ssi::SsShip> for SsShipInfo {
    fn from(ss_ship: &ssi::SsShip) -> Self {
        SsShipInfo::new(ss_ship.id, ss_ship.fit_id, ss_ship.a_item_id, ss_ship.get_bool_state())
    }
}
