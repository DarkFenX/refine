use crate::{
    num::{PValue, UnitInterval, Value},
    rd::RAttrId,
    svc::{SvcCtx, calc::Calc},
    ud::UItemId,
};

pub(in crate::svc::vast::stats) fn calc_regen_for_attrs(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    max_attr_rid: Option<RAttrId>,
    regen_attr_rid: Option<RAttrId>,
    resource_perc: UnitInterval,
) -> PValue {
    let regen_duration_ms = calc.get_item_oattr_ffb_extra(ctx, item_uid, regen_attr_rid, Value::ZERO);
    let regen_duration_s = match regen_duration_ms < Value::FLOAT_TOLERANCE {
        true => return PValue::ZERO,
        false => PValue::from_value_clamped(regen_duration_ms / Value::THOUSAND),
    };
    let resource_max =
        PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(ctx, item_uid, max_attr_rid, Value::ZERO));
    calc_regen(resource_max, regen_duration_s, resource_perc)
}

fn calc_regen(resource_max: PValue, recharge_duration: PValue, resource_perc: UnitInterval) -> PValue {
    let resource_perc = resource_perc.into_pvalue();
    PValue::TEN * resource_max / recharge_duration * PValue::from_value_unchecked(resource_perc.sqrt() - resource_perc)
}

pub(in crate::svc::vast::stats) fn regenerate(
    c0: PValue,
    c_max: PValue,
    tau: PValue,
    t0: PValue,
    t1: PValue,
) -> PValue {
    (Value::ONE + ((c0 / c_max).sqrt() - PValue::ONE) * ((t0 - t1) / tau).exp()).pow2() * c_max
}
