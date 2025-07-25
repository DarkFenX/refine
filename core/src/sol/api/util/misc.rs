use crate::{ac, rd, ud::UItem};

pub(super) fn is_a_effect_projectable(u_item: &UItem, a_effect: &rd::REffect) -> bool {
    // Projected effects do not apply targeted effects. Projected effects are item-targetable
    // version of system-wide and fit-wide effects for more granular application; so they work only
    // with system effects and buff effects
    (a_effect.get_category() == ac::effcats::TARGET && !matches!(u_item, UItem::ProjEffect(_)))
        || (a_effect.get_category() == ac::effcats::SYSTEM
            && matches!(u_item, UItem::SwEffect(_) | UItem::FwEffect(_) | UItem::ProjEffect(_)))
        || a_effect.get_a_buff_info().is_some()
}
