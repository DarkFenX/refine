use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectGeneralOutputGetter, NEffectNeut, NEffectNeutKind, NEffectProjMultGetter, NEffectProjOpcSpec,
        NEffectResist,
    },
};

const EFFECT_EID: EEffectId = EEffectId::STRUCT_ENERGY_NEUT_FALLOFF;
const EFFECT_AID: AEffectId = AEffectId::STRUCT_ENERGY_NEUT_FALLOFF;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        neut: Some(NEffectNeut {
            kind: NEffectNeutKind::Module,
            checker: None,
            ospec: NEffectProjOpcSpec {
                base: NEffectGeneralOutputGetter::Neut,
                proj_mult_str: Some(NEffectProjMultGetter::Neut),
                resist: Some(NEffectResist::Standard),
                limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
                ..
            },
        }),
        ..
    }
}
