use crate::{
    defs::{EItemId, SsFitId, SsItemId},
    ss::item::SsShip,
};

pub struct SsShipInfo {
    pub id: SsItemId,
    pub fit_id: SsFitId,
    pub a_item_id: EItemId,
    pub enabled: bool,
}
impl SsShipInfo {
    fn new(id: SsItemId, fit_id: SsFitId, a_item_id: EItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SsShip> for SsShipInfo {
    fn from(ss_ship: &SsShip) -> Self {
        SsShipInfo::new(ss_ship.id, ss_ship.fit_id, ss_ship.a_item_id, ss_ship.get_bool_state())
    }
}
