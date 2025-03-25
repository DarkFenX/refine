use crate::{
    ac, ad,
    sol::{svc::calc::modifier::RawModifier, uad::item::Item},
};

use super::{aar, prop};

pub(in crate::sol::svc::calc) fn extend_with_custom_mods(
    item: &Item,
    a_effect_id: ad::AEffectId,
    mods: &mut Vec<RawModifier>,
) {
    match a_effect_id {
        ac::effects::REE_AAR_PASTE_BOOST => mods.push(aar::make_mod(item.get_item_id(), a_effect_id)),
        ac::effects::MOD_BONUS_AFTERBURNER => mods.push(prop::make_mod(item.get_item_id(), a_effect_id)),
        ac::effects::MOD_BONUS_MICROWARPDRIVE => mods.push(prop::make_mod(item.get_item_id(), a_effect_id)),
        _ => (),
    }
}
