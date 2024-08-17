use crate::{ad, ec};

pub(super) fn is_effect_projectable(effect: &ad::AEffect) -> bool {
    effect.category == ec::effcats::TARGET || effect.category == ec::effcats::SYSTEM || effect.buff.is_some()
}
