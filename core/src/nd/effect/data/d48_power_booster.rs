use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeDeplChargeRate, NEffectChargeLoc, NEffectLocalOpcSpec,
    },
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::POWER_BOOSTER;
const A_EFFECT_ID: AEffectId = ac::effects::POWER_BOOSTER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate { .. })),
            activates_charge: false,
        }),
        cap_inject_opc_spec: Some(NEffectLocalOpcSpec {
            base: internal_get_cap_inject,
            ilimit_attr_id: Some(ac::attrs::CAPACITOR_CAPACITY),
            ..
        }),
        ..
    }
}

fn internal_get_cap_inject(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    _effect: &REffect,
) -> Option<Output<AttrVal>> {
    let item = ctx.u_data.items.get(item_key);
    let charge_key = item.get_charge_key()?;
    let attr_consts = ctx.ac();
    let amount = calc.get_item_oattr_afb_oextra(ctx, charge_key, attr_consts.capacitor_bonus, OF(0.0))?;
    Some(Output::Simple(OutputSimple { amount, delay: OF(0.0) }))
}
