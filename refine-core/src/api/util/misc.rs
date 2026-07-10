use crate::{ad::AEffectCatId, rd::REffect, ud::UItem};

pub(super) fn is_effect_projectable(u_item: &UItem, effect: &REffect) -> bool {
    // Projected effect item type does not apply targeted effects. They are item-targetable version
    // of system-wide and fit-wide effects for more granular application; so they work only with
    // system effects and buff effects
    (matches!(effect.category, AEffectCatId::ACTIVE | AEffectCatId::TARGET) && !matches!(u_item, UItem::ProjEffect(_)))
        || (effect.category == AEffectCatId::SYSTEM
            && matches!(u_item, UItem::SwEffect(_) | UItem::FwEffect(_) | UItem::ProjEffect(_)))
        || effect.buff.is_some()
}
