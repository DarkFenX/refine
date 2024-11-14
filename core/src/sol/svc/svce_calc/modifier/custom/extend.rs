use crate::{
    ec,
    sol::{item::SolItem, svc::svce_calc::modifier::SolRawModifier},
    EEffectId,
};

use super::{aar, prop};

pub(in crate::sol::svc::svce_calc) fn extend_with_custom_mods(
    item: &SolItem,
    effect_id: EEffectId,
    mods: &mut Vec<SolRawModifier>,
) {
    match effect_id {
        ec::effects::REE_AAR_PASTE_BOOST => mods.push(aar::make_mod(item.get_id(), effect_id)),
        ec::effects::MOD_BONUS_AFTERBURNER => mods.push(prop::make_mod(item.get_id(), effect_id)),
        ec::effects::MOD_BONUS_MICROWARPDRIVE => mods.push(prop::make_mod(item.get_id(), effect_id)),
        _ => (),
    }
}
