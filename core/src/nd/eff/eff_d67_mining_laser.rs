use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectHc},
};

const E_EFFECT_ID: EEffectId = ec::effects::MINING_LASER;
const A_EFFECT_ID: AEffectId = ac::effects::MINING_LASER;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            charge: Some(NEffectCharge {
                location: NEffectChargeLoc::Loaded(NEffectChargeDepl::Crystal {
                    can_run_uncharged: true,
                }),
                activates_charge: false,
            }),
            ..
        },
        ..
    }
}
