use crate::{
    num::{PValue, Value},
    svc::{SvcCtx, calc::Calc, vast::StatDmgBreacher},
    ud::UItemId,
};

pub(in crate::svc::vast::vaste_stats::dmg) fn apply_breacher(
    ctx: SvcCtx,
    calc: &mut Calc,
    breacher_raw: StatDmgBreacher,
    projectee_uid: UItemId,
) -> PValue {
    let hp_shield = PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
        ctx,
        projectee_uid,
        ctx.ac().shield_capacity,
        Value::ZERO,
    ));
    let hp_armor =
        PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(ctx, projectee_uid, ctx.ac().armor_hp, Value::ZERO));
    let hp_hull =
        PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(ctx, projectee_uid, ctx.ac().hp, Value::ZERO));
    breacher_raw
        .absolute_max
        .min(breacher_raw.relative_max * (hp_shield + hp_armor + hp_hull))
}
