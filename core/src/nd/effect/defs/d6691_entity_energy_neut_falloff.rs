use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectGeneralOutputGetter, NEffectNeut, NEffectNeutKind, NEffectProjMultGetter, NEffectProjOpcSpec,
        NEffectResist,
    },
};

const EFFECT_EID: EEffectId = EEffectId::ENTITY_ENERGY_NEUT_FALLOFF;
const EFFECT_AID: AEffectId = AEffectId::ENTITY_ENERGY_NEUT_FALLOFF;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        neut: Some(NEffectNeut {
            kind: NEffectNeutKind::Minion,
            checker: None,
            ospec: NEffectProjOpcSpec {
                base: NEffectGeneralOutputGetter::Neut,
                proj_mult_str: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
                resist: Some(NEffectResist::Standard),
                remote_limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
                ..
            },
        }),
        ..
    }
}
