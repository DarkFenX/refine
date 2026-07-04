use super::shared::add_tp_mods;
use crate::{
    ad::AEffectId,
    nd::{NEffect, NEffectProjGetter, NEffectProjModSpec},
};

const EFFECT_AID: AEffectId = AEffectId::STRUCT_MOD_EFFECT_TARGET_PAINTER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(|a_effect, adg_warnings| add_tp_mods(EFFECT_AID, a_effect, adg_warnings)),
        proj_mod: Some(NEffectProjModSpec {
            proj_mult: Some(NEffectProjGetter::GenericRangeFullStsRestricted),
            ..
        }),
        ..
    }
}
