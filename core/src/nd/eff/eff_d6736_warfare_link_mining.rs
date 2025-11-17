use crate::{
    ac,
    ad::{AEffectBuffInfo, AEffectBuffScope, AEffectBuffSrc, AEffectId},
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectHc},
};

const E_EFFECT_ID: EEffectId = ec::effects::MOD_BONUS_WARFARE_LINK_MINING;
const A_EFFECT_ID: AEffectId = ac::effects::MOD_BONUS_WARFARE_LINK_MINING;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            source: AEffectBuffSrc::DefaultAttrs,
            scope: AEffectBuffScope {
                item_list_id: ac::itemlists::SHIPS,
                fleet_only: true,
                ..
            },
        }),
        hc: NEffectHc {
            charge: Some(NEffectCharge {
                location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate {
                    can_run_uncharged: false,
                }),
                activates_charge: false,
            }),
            ..
        },
        ..
    }
}
