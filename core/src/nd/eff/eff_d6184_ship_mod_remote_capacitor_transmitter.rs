use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::{
            opc_rep::get_remote_cap_rep_opc,
            proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
        },
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIP_MOD_REMOTE_CAPACITOR_TRANSMITTER;
const A_EFFECT_ID: AEffectId = ac::effects::SHIP_MOD_REMOTE_CAPACITOR_TRANSMITTER;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            proj_mult_getter: Some(get_proj_mult_simple_s2s),
            remote_cap_rep_opc_getter: Some(get_remote_cap_rep_opc),
            ..
        },
        ..
    }
}
