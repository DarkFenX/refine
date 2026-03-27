use super::shared::{get_aoe_dd_side_neut_ospec, make_dd_self_debuffs};
use crate::{
    ad::{AEffectBuff, AEffectId},
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_EID: EEffectId = EEffectId::DOOMSDAY_HOG;
const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_HOG;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: make_dd_self_debuffs().collect(),
            ..
        }),
        neut_opc_spec: Some(get_aoe_dd_side_neut_ospec()),
        ..
    }
}
