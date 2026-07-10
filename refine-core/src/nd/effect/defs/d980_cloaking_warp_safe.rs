use super::shared::mk_stabilize_cloak_buff;
use crate::{
    ad::{AEffectBuff, AEffectId},
    nd::{NEffect, NEffectDuration},
};

const EFFECT_AID: AEffectId = AEffectId::CLOAKING_WARP_SAFE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: vec![mk_stabilize_cloak_buff()],
            ..
        }),
        cloaks_carrier: true,
        disallows_jump_gate: Some(NEffectDuration::Effect),
        disallows_jump_wh: Some(NEffectDuration::Effect),
        disallows_jump_drive: Some(NEffectDuration::Effect),
        disallows_dock: Some(NEffectDuration::Effect),
        disallows_tether: Some(NEffectDuration::Effect),
        ..
    }
}
