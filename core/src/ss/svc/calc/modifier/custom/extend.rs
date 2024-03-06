use crate::{
    defs::{EEffectId, SsItemId},
    ec,
    ss::svc::calc::modifier::SsAttrMod,
};

use super::aar;

pub(in crate::ss::svc::calc) fn extend_with_custom_mods(
    item_id: SsItemId,
    effect_id: EEffectId,
    mods: &mut Vec<SsAttrMod>,
) {
    match effect_id {
        ec::effects::REE_AAR_PASTE_BOOST => mods.push(aar::make_mod(item_id, effect_id)),
        _ => (),
    }
}
