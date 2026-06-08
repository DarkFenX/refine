use super::shared::add_damp_mods;
use crate::{
    ad::AEffectId,
    nd::{NEffect, NEffectProjGetter, NEffectProjModSpec},
};

const EFFECT_AID: AEffectId = AEffectId::STRUCT_MOD_EFFECT_REMOTE_SENSOR_DAMPENER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(|a_effect| add_damp_mods(EFFECT_AID, a_effect)),
        proj_mod: Some(NEffectProjModSpec {
            proj_mult: NEffectProjGetter::GenericRangeFullStsRestricted,
            ..
        }),
        ..
    }
}
