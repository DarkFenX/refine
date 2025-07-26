use crate::{
    def::{AttrVal, OF},
    misc::EffectSpec,
    rd,
    svc::{SvcCtx, calc::Calc},
    ud::UItemKey,
};

pub(crate) fn get_espec_duration_s(ctx: SvcCtx, calc: &mut Calc, espec: EffectSpec) -> Option<AttrVal> {
    let r_effect = ctx.u_data.src.get_effect(espec.effect_key);
    get_effect_duration_s(ctx, calc, espec.item_key, r_effect)
}

pub(crate) fn get_effect_duration_s(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    r_effect: &rd::REffect,
) -> Option<AttrVal> {
    let attr_id = r_effect.get_duration_attr_id()?;
    let val = calc.get_item_attr_val_full(ctx, item_key, &attr_id).ok()?;
    // Discard negative cycle time as invalid
    match val.dogma > OF(0.0) {
        true => Some(val.extra / OF(1000.0)),
        false => None,
    }
}
