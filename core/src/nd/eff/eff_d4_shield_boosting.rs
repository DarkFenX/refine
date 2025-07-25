use crate::{
    ac, ec,
    nd::{NEffect, NEffectHc, eff::shared::opc_rep::get_local_shield_rep_opc},
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::SHIELD_BOOSTING),
        aid: ac::effects::SHIELD_BOOSTING,
        hc: NEffectHc {
            local_shield_rep_opc_getter: Some(get_local_shield_rep_opc),
            ..
        },
        ..
    }
}
