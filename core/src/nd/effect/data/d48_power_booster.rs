use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc},
    svc::{SvcCtx, calc::Calc},
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::POWER_BOOSTER;
const A_EFFECT_ID: AEffectId = ac::effects::POWER_BOOSTER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate {
                can_run_uncharged: false,
            }),
            activates_charge: false,
        }),
        cap_inject_getter: Some(get_cap_inject),
        ..
    }
}

fn get_cap_inject(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> Option<AttrVal> {
    let item = ctx.u_data.items.get(item_key);
    let charge_key = item.get_charge_key()?;
    let attr_consts = ctx.ac();
    let charge_amount = calc.get_item_oattr_afb_oextra(ctx, charge_key, attr_consts.capacitor_bonus, OF(0.0))?;
    let fit_key = ctx.u_data.items.get(charge_key).dc_charge().unwrap().get_fit_key();
    let ship_key = ctx.u_data.fits.get(fit_key).ship;
    let ship_amount = calc.get_oitem_oattr_ffb_extra(ctx, ship_key, attr_consts.capacitor_capacity, OF(0.0));
    let amount = AttrVal::min(charge_amount, ship_amount);
    Some(amount)
}
