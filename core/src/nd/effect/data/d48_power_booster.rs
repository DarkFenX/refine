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
    ud::UItemId,
};

const EFFECT_EID: EEffectId = ec::effects::POWER_BOOSTER;
const EFFECT_AID: AEffectId = ac::effects::POWER_BOOSTER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(NEffectChargeDeplChargeRate { .. })),
            activates_charge: false,
        }),
        cap_inject_opc_spec: Some(NEffectLocalOpcSpec {
            base: internal_get_cap_inject,
            limit_attr_id: Some(ac::attrs::CAPACITOR_CAPACITY),
            ..
        }),
        ..
    }
}

fn internal_get_cap_inject(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    _effect: &REffect,
) -> Option<Output<AttrVal>> {
    let item = ctx.u_data.items.get(item_uid);
    let charge_uid = item.get_charge_uid()?;
    let attr_consts = ctx.ac();
    let amount = calc.get_item_oattr_afb_oextra(ctx, charge_uid, attr_consts.capacitor_bonus, OF(0.0))?;
    Some(Output::Simple(OutputSimple { amount, delay: OF(0.0) }))
}
