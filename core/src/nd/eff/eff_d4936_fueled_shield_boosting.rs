use crate::{
    ac, ec,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectHc,
        eff::shared::opc_rep::get_local_shield_rep_opc,
    },
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::FUELED_SHIELD_BOOSTING),
        aid: ac::effects::FUELED_SHIELD_BOOSTING,
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
