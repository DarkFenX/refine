use super::attr_val::get_volume;
use crate::{
    ac,
    ad::AItemCatId,
    misc::{PValue, Value},
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

#[derive(Copy, Clone)]
pub(crate) enum RShipKind {
    Ship,
    CapitalShip,
    Structure,
}

const MAX_SUBCAP_MODULE_VOLUME: PValue = PValue::from_f64_clamped(ac::extras::MAX_SUBCAP_MODULE_VOLUME);

pub(in crate::rd::data::item::attr_extras) fn get_item_ship_kind(
    item_cat_id: AItemCatId,
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<RShipKind> {
    match item_cat_id {
        ac::itemcats::MODULE => match get_volume(item_attrs, attr_consts) <= MAX_SUBCAP_MODULE_VOLUME {
            true => Some(RShipKind::Ship),
            false => Some(RShipKind::CapitalShip),
        },
        ac::itemcats::STRUCTURE_MODULE => Some(RShipKind::Structure),
        _ => None,
    }
}
