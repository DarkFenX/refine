use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectCharge, NEffectChargeLoc, NEffectHc},
};

const E_EFFECT_ID: EEffectId = ec::effects::FTR_ABIL_BOMB;
const A_EFFECT_ID: AEffectId = ac::effects::FTR_ABIL_BOMB;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
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
