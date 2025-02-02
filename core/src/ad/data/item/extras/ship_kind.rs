use crate::{
    defs::{AttrVal, EAttrId, EItemCatId, EItemId, SkillLevel, OF},
    ec,
    util::StMap,
};

/// Adapted ship type.
#[derive(Copy, Clone)]
pub enum AShipKind {
    Ship,
    CapitalShip,
    Structure,
}

pub(super) fn get_ship_kind(cat_id: EItemCatId, srqs: &StMap<EItemId, SkillLevel>) -> Option<AShipKind> {
    match cat_id {
        ec::itemcats::SHIP => match srqs.contains_key(&ec::items::CAPITAL_SHIPS) {
            true => Some(AShipKind::CapitalShip),
            false => Some(AShipKind::Ship),
        },
        ec::itemcats::STRUCTURE => Some(AShipKind::Structure),
        _ => None,
    }
}

pub(super) fn get_item_ship_kind(cat_id: EItemCatId, attrs: &StMap<EAttrId, AttrVal>) -> Option<AShipKind> {
    match cat_id {
        ec::itemcats::MODULE => match attrs.get(&ec::attrs::VOLUME) {
            Some(&volume) => match volume <= OF(3500.0) {
                true => Some(AShipKind::Ship),
                false => Some(AShipKind::CapitalShip),
            },
            None => Some(AShipKind::Ship),
        },
        ec::itemcats::STRUCTURE_MODULE => Some(AShipKind::Structure),
        _ => None,
    }
}
