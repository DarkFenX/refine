use super::attr_val::get_volume;
use crate::{
    ac,
    ad::{AAttrId, AAttrVal, AItemCatId},
    util::RMap,
};

#[derive(Copy, Clone)]
pub(crate) enum RShipKind {
    Ship,
    CapitalShip,
    Structure,
}

pub(super) fn get_item_ship_kind(item_cat_id: AItemCatId, item_attrs: &RMap<AAttrId, AAttrVal>) -> Option<RShipKind> {
    match item_cat_id {
        ac::itemcats::MODULE => match get_volume(item_attrs) <= ac::extras::MAX_SUBCAP_MODULE_VOLUME {
            true => Some(RShipKind::Ship),
            false => Some(RShipKind::CapitalShip),
        },
        ac::itemcats::STRUCTURE_MODULE => Some(RShipKind::Structure),
        _ => None,
    }
}
