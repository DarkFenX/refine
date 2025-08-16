use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectHc,
        eff::shared::rep_opc::get_local_shield_rep_opc,
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::FUELED_SHIELD_BOOSTING;
const A_EFFECT_ID: AEffectId = ac::effects::FUELED_SHIELD_BOOSTING;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            charge: Some(NEffectCharge {
                location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate {
                    can_run_uncharged: true,
                }),
                activates_charge: false,
            }),
            local_shield_rep_opc_getter: Some(get_local_shield_rep_opc),
            ..
        },
        ..
    }
}
