use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectHc, eff::shared::mining_opc},
};

const E_EFFECT_ID: EEffectId = ec::effects::MINING_CLOUDS;
const A_EFFECT_ID: AEffectId = ac::effects::MINING_CLOUDS;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            mining_gas_opc_getter: Some(mining_opc::get_mining_opc),
            ..
        },
        ..
    }
}
