use crate::{
    misc::EffectSpec,
    num::{PValue, Value},
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::UItemId,
};

pub(crate) fn get_espec_duration_s(ctx: SvcCtx, calc: &mut Calc, espec: EffectSpec) -> Option<PValue> {
    let effect = ctx.u_data.src.get_effect_by_rid(espec.effect_rid);
    get_effect_duration_s(ctx, calc, espec.item_uid, effect)
}

pub(crate) fn get_effect_duration_s(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
) -> Option<PValue> {
    let val = calc.get_item_oattr_oextra(ctx, item_uid, effect.duration_attr_rid)?;
    // Discard zero / negative cycle time as invalid
    match val > Value::FLOAT_TOLERANCE {
        true => Some(PValue::from_f64_unchecked(val.into_f64() / 1000.0)),
        false => None,
    }
}
