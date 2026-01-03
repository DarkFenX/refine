use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectProjOpcSpec, NEffectResist,
        effect::data::shared::{base_opc::get_neut_base_opc, proj_mult::get_neut_proj_mult},
    },
};

const EFFECT_EID: EEffectId = ec::effects::ENERGY_NEUT_FALLOFF;
const EFFECT_AID: AEffectId = ac::effects::ENERGY_NEUT_FALLOFF;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        neut_opc_spec: Some(NEffectProjOpcSpec {
            base: get_neut_base_opc,
            proj_mult_str: Some(get_neut_proj_mult),
            resist: Some(NEffectResist::Standard),
            limit_attr_id: Some(ac::attrs::CAPACITOR_CAPACITY),
            ..
        }),
        ..
    }
}
