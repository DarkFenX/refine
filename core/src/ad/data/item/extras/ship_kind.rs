use crate::{
    ac,
    ad::{AAttrId, AAttrVal, AItemCatId, AItemId, ASkillLevel},
    util::HMap,
};

/// Adapted ship type.
#[derive(Copy, Clone)]
pub enum AShipKind {
    Ship,
    CapitalShip,
    Structure,
}

pub(super) fn get_ship_kind(item_cat_id: AItemCatId, item_srqs: &HMap<AItemId, ASkillLevel>) -> Option<AShipKind> {
    match item_cat_id {
        ac::itemcats::SHIP => match item_srqs.contains_key(&ac::items::CAPITAL_SHIPS) {
            true => Some(AShipKind::CapitalShip),
            false => Some(AShipKind::Ship),
        },
        ac::itemcats::STRUCTURE => Some(AShipKind::Structure),
        _ => None,
    }
}

pub(super) fn get_item_ship_kind(item_cat_id: AItemCatId, item_attrs: &HMap<AAttrId, AAttrVal>) -> Option<AShipKind> {
    match item_cat_id {
        ac::itemcats::MODULE => match item_attrs.get(&ac::attrs::VOLUME) {
            Some(&volume) => match volume <= ac::extras::MAX_SUBCAP_MODULE_VOLUME {
                true => Some(AShipKind::Ship),
                false => Some(AShipKind::CapitalShip),
            },
            None => Some(AShipKind::Ship),
        },
        ac::itemcats::STRUCTURE_MODULE => Some(AShipKind::Structure),
        _ => None,
    }
}
