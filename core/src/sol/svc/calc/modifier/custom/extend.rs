use crate::{
    EEffectId, ec,
    sol::{svc::calc::modifier::SolRawModifier, uad::item::SolItem},
};

use super::{aar, prop};

pub(in crate::sol::svc::calc) fn extend_with_custom_mods(
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
