use super::shared::add_tp_mods;
use crate::{
    ad::AEffectId,
    nd::{NEffect, NEffectProjGetter},
};

const EFFECT_AID: AEffectId = AEffectId::REMOTE_TARGET_PAINT_ENTITY;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(|a_effect| add_tp_mods(EFFECT_AID, a_effect)),
        modifier_proj: Some(NEffectProjGetter::GenericRangeSimpleSts),
        ..
    }
}
