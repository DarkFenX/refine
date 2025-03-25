use crate::{ac, ad};

pub(super) fn is_a_effect_projectable(a_effect: &ad::AEffect) -> bool {
    a_effect.category == ac::effcats::TARGET || a_effect.category == ac::effcats::SYSTEM || a_effect.buff.is_some()
}
