use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{NEffect, NEffectGeneralOutputGetter, NEffectProjGetter, NEffectProjOpcSpec, NEffectResist},
};

const EFFECT_EID: EEffectId = EEffectId::SHIP_MOD_REMOTE_CAPACITOR_TRANSMITTER;
const EFFECT_AID: AEffectId = AEffectId::SHIP_MOD_REMOTE_CAPACITOR_TRANSMITTER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        outgoing_cap: Some(NEffectProjOpcSpec {
            base: NEffectGeneralOutputGetter::PowerTransfer,
            proj_mult_str: Some(NEffectProjGetter::GenericRangeSimpleSts),
            resist: Some(NEffectResist::Standard),
            remote_limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
            ..
        }),
        ..
    }
}
