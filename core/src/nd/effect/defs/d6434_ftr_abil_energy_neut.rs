use crate::{
    ad::{AAttrId, AEffectId},
    nd::{
        NEffect, NEffectGeneralOutputGetter, NEffectNeut, NEffectNeutKind, NEffectProjGetter, NEffectProjOpcSpec,
        NEffectResist,
    },
};

const EFFECT_AID: AEffectId = AEffectId::FTR_ABIL_ENERGY_NEUT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        neut: Some(NEffectNeut {
            kind: NEffectNeutKind::Minion,
            checker: None,
            ospec: NEffectProjOpcSpec {
                base: NEffectGeneralOutputGetter::NeutFtrAbil,
                proj_mult_str: Some(NEffectProjGetter::GenericRangeFullStsRestricted),
                resist: Some(NEffectResist::Standard),
                remote_limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
                ..
            },
        }),
        ..
    }
}
