use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::{
            opc_rep::get_remote_armor_rep_opc,
            proj_mult::{get_proj_attrs_normal, get_proj_mult_normal_restricted_s2s},
        },
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIP_MOD_REMOTE_ARMOR_REPAIRER;
const A_EFFECT_ID: AEffectId = ac::effects::SHIP_MOD_REMOTE_ARMOR_REPAIRER;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        xt_get_proj_attrs: Some(get_proj_attrs_normal),
        hc: NEffectHc {
            proj_mult_getter: Some(get_proj_mult_normal_restricted_s2s),
            remote_armor_rep_opc_getter: Some(get_remote_armor_rep_opc),
            ..
        },
        ..
    }
}
