use crate::{ac, rd, ud::UItem};

pub(super) fn is_a_effect_projectable(u_item: &UItem, a_effect: &rd::REffect) -> bool {
    // Projected effects do not apply targeted effects. Projected effects are item-targetable
    // version of system-wide and fit-wide effects for more granular application; so they work only
    // with system effects and buff effects
    (matches!(a_effect.category, AEffectCatId::ACTIVE | AEffectCatId::TARGET)
        && !matches!(u_item, UItem::ProjEffect(_)))
        || (a_effect.category == AEffectCatId::SYSTEM
            && matches!(u_item, UItem::SwEffect(_) | UItem::FwEffect(_) | UItem::ProjEffect(_)))
        || a_effect.buff.is_some()
}
