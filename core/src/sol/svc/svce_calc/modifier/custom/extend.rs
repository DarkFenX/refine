use crate::{
    defs::{EEffectId, SolItemId},
    ec,
    sol::svc::svce_calc::modifier::SolModifier,
};

use super::{aar, prop};

pub(in crate::sol::svc::svce_calc) fn extend_with_custom_mods(
    item_id: SolItemId,
    effect_id: EEffectId,
    mods: &mut Vec<SolModifier>,
) {
    match effect_id {
        ec::effects::REE_AAR_PASTE_BOOST => mods.push(aar::make_mod(item_id, effect_id)),
        ec::effects::MOD_BONUS_AFTERBURNER => mods.push(prop::make_mod(item_id, effect_id)),
        ec::effects::MOD_BONUS_MICROWARPDRIVE => mods.push(prop::make_mod(item_id, effect_id)),
        _ => (),
    }
}
