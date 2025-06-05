use ordered_float::OrderedFloat as OF;

use crate::{
    ad,
    sol::{
        AttrVal,
        svc::{AttrSpec, calc::Calc},
        uad::{Uad, item::UadItem},
    },
};

pub(in crate::sol::svc) fn get_resist_a_attr_id(item: &UadItem, a_effect: &ad::AEffect) -> Option<ad::AAttrId> {
    match a_effect.resist_attr_id {
        Some(resist_a_attr_id) => Some(resist_a_attr_id),
        None => match item.get_a_extras() {
            Some(a_extras) => a_extras.remote_resist_attr_id,
            None => None,
        },
    }
}

pub(in crate::sol::svc) fn get_resist_mult_val(uad: &Uad, calc: &mut Calc, aspec: &AttrSpec) -> Option<AttrVal> {
    let mult = calc
        .get_item_attr_val_full(uad, aspec.item_key, &aspec.a_attr_id)
        .ok()?
        .dogma;
    Some(match mult.abs() <= 0.0001 {
        true => OF(0.0),
        false => mult,
    })
}
