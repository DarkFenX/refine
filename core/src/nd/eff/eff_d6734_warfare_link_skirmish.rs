use crate::{
    ac, ad, ec,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectHc},
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::MOD_BONUS_WARFARE_LINK_SKIRMISH),
        aid: ac::effects::MOD_BONUS_WARFARE_LINK_SKIRMISH,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::FleetShips,
        }),
        hc: NEffectHc {
            charge: Some(NEffectCharge::Loaded(NEffectChargeDepl::ChargeRate)),
            ..
        },
        ..
    }
}
