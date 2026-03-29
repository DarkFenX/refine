use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectGeneralOutputGetter, NEffectNeut, NEffectNeutKind, NEffectProjMultGetter, NEffectProjOpcSpec,
        NEffectResist,
    },
};

const EFFECT_EID: EEffectId = EEffectId::DOOMSDAY_AOE_NEUT;
const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_NEUT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        neut: Some(NEffectNeut {
            kind: NEffectNeutKind::Module,
            ospec: NEffectProjOpcSpec {
                base: NEffectGeneralOutputGetter::NeutAoe,
                proj_mult_str: Some(NEffectProjMultGetter::AoeBurst),
                resist: Some(NEffectResist::Standard),
                limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
                ..
            },
        }),
        ..
    }
}
