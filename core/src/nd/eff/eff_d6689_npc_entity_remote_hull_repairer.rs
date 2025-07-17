use crate::{
    ac, ec,
    nd::{
        NEffect, NEffectHc,
        eff::shared::{
            opc_rep::get_remote_hull_rep_opc,
            proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
        },
    },
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::NPC_ENTITY_REMOTE_HULL_REPAIRER),
        aid: ac::effects::NPC_ENTITY_REMOTE_HULL_REPAIRER,
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            get_proj_mult: Some(get_proj_mult_simple_s2s),
            get_remote_hull_rep_opc: Some(get_remote_hull_rep_opc),
            ..
        },
        ..
    }
}
