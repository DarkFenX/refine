use crate::{ad, consts};

pub(super) fn is_effect_projectable(effect: &ad::AEffect) -> bool {
    effect.category == consts::effcats::TARGET || effect.category == consts::effcats::SYSTEM || effect.buff.is_some()
}
