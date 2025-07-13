use crate::{
    ac, ec,
    nd::{NEffect, NEffectHc, eff::shared::rep_amount::get_local_hull_rep_amount},
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::STRUCTURE_REPAIR),
        aid: ac::effects::STRUCTURE_REPAIR,
        hc: NEffectHc {
            get_local_hull_rep_amount: Some(get_local_hull_rep_amount),
            ..
        },
        ..
    }
}
