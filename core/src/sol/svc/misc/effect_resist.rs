use ordered_float::OrderedFloat as OF;

use crate::{
    ad,
    sol::{
        AttrVal, ItemKey,
        svc::{AttrSpec, EffectSpec, calc::Calc, eprojs::EProjs},
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

pub(in crate::sol::svc) fn get_resist_mult_val_by_projectee_aspec(
    uad: &Uad,
    eprojs: &EProjs,
    calc: &mut Calc,
    projectee_aspec: &AttrSpec,
) -> Option<AttrVal> {
    let mult = calc
        .get_item_attr_val_full(uad, eprojs, projectee_aspec.item_key, &projectee_aspec.a_attr_id)
        .ok()?
        .dogma;
    Some(match mult.abs() <= 0.0001 {
        true => OF(0.0),
        false => mult,
    })
}

pub(in crate::sol::svc) fn get_resist_mult_val(
    uad: &Uad,
    eprojs: &EProjs,
    calc: &mut Calc,
    projector_espec: &EffectSpec,
    projectee_item_key: ItemKey,
) -> Option<AttrVal> {
    let projector_a_effect = uad.src.get_a_effect(&projector_espec.a_effect_id)?;
    let projector_item = uad.items.get(projector_espec.item_key);
    let resist_a_attr_id = get_resist_a_attr_id(projector_item, projector_a_effect)?;
    get_resist_mult_val_by_projectee_aspec(uad, eprojs, calc, &AttrSpec::new(projectee_item_key, resist_a_attr_id))
}
