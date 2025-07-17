use crate::{
    ac, ad, ec,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectHc},
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::MOD_BONUS_WARFARE_LINK_ARMOR),
        aid: ac::effects::MOD_BONUS_WARFARE_LINK_ARMOR,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::FleetShips,
        }),
        hc: NEffectHc {
            charge: Some(NEffectCharge::Loaded(NEffectChargeDepl::ChargeRate {
                can_run_uncharged: false,
            })),
            ..
        },
        ..
    }
}
