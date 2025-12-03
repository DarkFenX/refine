use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectHc, NEffectProjecteeFilter},
};

const E_EFFECT_ID: EEffectId = ec::effects::USE_MISSILES;
const A_EFFECT_ID: AEffectId = ac::effects::USE_MISSILES;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            charge: Some(NEffectCharge {
                location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate {
                    can_run_uncharged: false,
                }),
                activates_charge: true,
            }),
            projectee_filter: Some(NEffectProjecteeFilter::ItemListAttr(ac::attrs::VALID_TGT_WHITELIST)),
            ..
        },
        ..
    }
}
