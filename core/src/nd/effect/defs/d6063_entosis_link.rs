use super::shared::mk_disallow_cloak_buff;
use crate::{
    ad::{AEffectBuff, AEffectId},
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::ENTOSIS_LINK;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: vec![mk_disallow_cloak_buff()],
            ..
        }),
        ..
    }
}
