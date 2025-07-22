use crate::{
    ac, ec,
    nd::{NEffect, NEffectCharge, NEffectChargeLoc, NEffectHc},
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::FTR_ABIL_BOMB),
        aid: ac::effects::FTR_ABIL_BOMB,
        hc: NEffectHc {
            charge: Some(NEffectCharge {
                location: NEffectChargeLoc::Autocharge(ac::attrs::FTR_ABIL_BOMB_TYPE),
                activates_charge: true,
            }),
            ..
        },
        ..
    }
}
