use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectMiningOutputGetter, NEffectProjMultGetter, NEffectProjOpcSpec},
};

const EFFECT_EID: EEffectId = EEffectId::MINING;
const EFFECT_AID: AEffectId = AEffectId::MINING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        mining_ore_opc_spec: Some(NEffectProjOpcSpec {
            base: NEffectMiningOutputGetter::MiningOre,
            proj_mult_str: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
            ..
        }),
        mining_ice_opc_spec: Some(NEffectProjOpcSpec {
            base: NEffectMiningOutputGetter::MiningIce,
            proj_mult_str: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
            ..
        }),
        ..
    }
}
