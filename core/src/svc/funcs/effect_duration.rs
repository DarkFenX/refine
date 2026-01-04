use crate::{
    def::{AttrVal, OF},
    misc::EffectSpec,
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::UItemId,
    util::FLOAT_TOLERANCE,
};

pub(crate) fn get_espec_duration_s(ctx: SvcCtx, calc: &mut Calc, espec: EffectSpec) -> Option<AttrVal> {
    let effect = ctx.u_data.src.get_effect_by_rid(espec.effect_rid);
    get_effect_duration_s(ctx, calc, espec.item_uid, effect)
}

pub(crate) fn get_effect_duration_s(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemId,
    effect: &REffect,
) -> Option<AttrVal> {
    let val = calc.get_item_oattr_oextra(ctx, item_key, effect.duration_attr_rid)?;
    // Discard negative cycle time as invalid
    match val > FLOAT_TOLERANCE {
        true => Some(val / OF(1000.0)),
        false => None,
    }
}
