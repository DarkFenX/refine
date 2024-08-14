use crate::{
    ad, ec,
    sol::{item::SolItem, svc::svce_calc::modifier::SolRawModifier},
};

use super::{aar, prop, web};

pub(in crate::sol::svc::svce_calc) fn extend_with_custom_mods(
    item: &SolItem,
    effect: &ad::AEffect,
    mods: &mut Vec<SolRawModifier>,
) {
    match effect.id {
        ec::effects::REE_AAR_PASTE_BOOST => mods.push(aar::make_mod(item.get_id(), effect.id)),
        ec::effects::MOD_BONUS_AFTERBURNER => mods.push(prop::make_mod(item.get_id(), effect.id)),
        ec::effects::MOD_BONUS_MICROWARPDRIVE => mods.push(prop::make_mod(item.get_id(), effect.id)),
        ec::effects::REMOTE_WEBIFIER_FALLOFF => mods.push(web::make_mod(item, effect)),
        ec::effects::STRUCTURE_MODULE_EFFECT_STASIS_WEBIFIER => mods.push(web::make_mod(item, effect)),
        ec::effects::REMOTE_WEBIFIER_ENTITY => mods.push(web::make_mod(item, effect)),
        _ => (),
    }
}
