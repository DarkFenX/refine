use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    misc::{PValue, Value},
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

const EFFECT_EID: EEffectId = EEffectId::POWER_BOOSTER;
const EFFECT_AID: AEffectId = AEffectId::POWER_BOOSTER;

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
            limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
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
) -> Option<Output<Value>> {
    let item = ctx.u_data.items.get(item_uid);
    let charge_uid = item.get_charge_uid()?;
    let attr_consts = ctx.ac();
    let amount = calc.get_item_oattr_afb_oextra(ctx, charge_uid, attr_consts.capacitor_bonus, Value::ZERO)?;
    Some(Output::Simple(OutputSimple {
        amount,
        delay: PValue::ZERO,
    }))
}
