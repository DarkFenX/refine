use super::attr_val::get_volume;
use crate::{
    ac,
    ad::{AAttrId, AAttrVal, AItemCatId, AItemId, ASkillLevel},
    util::RMap,
};

#[derive(Copy, Clone)]
pub(crate) enum AShipKind {
    Ship,
    CapitalShip,
    Structure,
}

pub(super) fn get_ship_kind(item_cat_id: AItemCatId, item_srqs: &RMap<AItemId, ASkillLevel>) -> Option<AShipKind> {
    match item_cat_id {
        ac::itemcats::SHIP => match item_srqs.contains_key(&ac::items::CAPITAL_SHIPS) {
            true => Some(AShipKind::CapitalShip),
            false => Some(AShipKind::Ship),
        },
        ac::itemcats::STRUCTURE => Some(AShipKind::Structure),
        _ => None,
    }
}

pub(super) fn get_item_ship_kind(item_cat_id: AItemCatId, item_attrs: &RMap<AAttrId, AAttrVal>) -> Option<AShipKind> {
    match item_cat_id {
        ac::itemcats::MODULE => match get_volume(item_attrs) <= ac::extras::MAX_SUBCAP_MODULE_VOLUME {
            true => Some(AShipKind::Ship),
            false => Some(AShipKind::CapitalShip),
        },
        ac::itemcats::STRUCTURE_MODULE => Some(AShipKind::Structure),
        _ => None,
    }
}
