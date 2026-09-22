use super::attr_val::get_volume;
use crate::{
    PValue, SkillLevel, Value,
    ad::{AItemCatId, AItemId},
    def::MAX_SUBCAP_MODULE_VOLUME,
    rd::{RAttrConsts, RAttrId, RShipKind},
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

pub(in crate::rd::data::item::base) fn get_item_ship_kind(
    item_cat_id: AItemCatId,
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<RShipKind> {
    match item_cat_id {
        AItemCatId::MODULE => {
            match get_volume(item_attrs, attr_consts) <= PValue::from_f64_clamped(MAX_SUBCAP_MODULE_VOLUME) {
                true => Some(RShipKind::Ship),
                false => Some(RShipKind::CapitalShip),
            }
        }
        AItemCatId::STRUCTURE_MODULE => Some(RShipKind::Structure),
        _ => None,
    }
}
