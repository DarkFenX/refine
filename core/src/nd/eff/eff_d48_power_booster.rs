use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectHc},
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::POWER_BOOSTER;
const A_EFFECT_ID: AEffectId = ac::effects::POWER_BOOSTER;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            charge: Some(NEffectCharge {
                location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate {
                    can_run_uncharged: false,
                }),
                activates_charge: false,
            }),
            cap_boost_opc_getter: Some(get_cap_boost_opc),
            ..
        },
        ..
    }
}

fn get_cap_boost_opc(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> Option<Output<AttrVal>> {
    let item = ctx.u_data.items.get(item_key);
    let charge_key = item.get_charge_key()?;
    let charge_amount = calc
        .get_item_attr_val_extra(ctx, charge_key, &ac::attrs::CAPACITOR_BONUS)
        .unwrap_or(OF(0.0));
    let fit_key = ctx.u_data.items.get(charge_key).get_charge().unwrap().get_fit_key();
    let ship_key = ctx.u_data.fits.get(fit_key).ship;
    let ship_amount = match ship_key {
        Some(ship_key) => calc
            .get_item_attr_val_extra(ctx, ship_key, &ac::attrs::CAPACITOR_CAPACITY)
            .unwrap_or(OF(0.0)),
        None => OF(0.0),
    };
    let amount = AttrVal::min(charge_amount, ship_amount);
    Some(Output::Simple(OutputSimple { amount, delay: OF(0.0) }))
}
