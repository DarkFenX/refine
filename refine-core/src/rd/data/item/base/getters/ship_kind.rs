use crate::{
    SkillLevel,
    ad::{AItemCatId, AItemId},
    rd::RShipKind,
    util::RMap,
};

pub(in crate::rd::data::item::base) fn get_ship_kind(
    item_cat_aid: AItemCatId,
    item_srqs: &RMap<AItemId, SkillLevel>,
) -> Option<RShipKind> {
    match item_cat_aid {
        AItemCatId::SHIP => match item_srqs.contains_key(&AItemId::CAPITAL_SHIPS) {
            true => Some(RShipKind::CapitalShip),
            false => Some(RShipKind::Ship),
        },
        AItemCatId::STRUCTURE => Some(RShipKind::Structure),
        _ => None,
    }
}
