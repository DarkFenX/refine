use crate::{ac, ad, uad::UadItem};

pub(super) fn is_a_effect_projectable(uad_item: &UadItem, a_effect: &ad::AEffectRt) -> bool {
    // Projected effects do not apply targeted effects. Projected effects are item-targetable
    // version of system-wide and fit-wide effects for more granular application; so they work only
    // with system effects and buff effects
    (a_effect.ae.category == ac::effcats::TARGET && !matches!(uad_item, UadItem::ProjEffect(_)))
        || (a_effect.ae.category == ac::effcats::SYSTEM
            && matches!(
                uad_item,
                UadItem::SwEffect(_) | UadItem::FwEffect(_) | UadItem::ProjEffect(_)
            ))
        || a_effect.ae.buff_info.is_some()
}
