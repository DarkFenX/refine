use crate::{
    ad,
    def::{AttrVal, ItemKey, OF},
    misc::EffectSpec,
    svc::{SvcCtx, calc::Calc},
};

pub(crate) fn get_espec_duration_s(ctx: SvcCtx, calc: &mut Calc, espec: EffectSpec) -> Option<AttrVal> {
    let a_effect = ctx.uad.src.get_a_effect(&espec.a_effect_id)?;
    get_effect_duration_s(ctx, calc, espec.item_key, a_effect)
}

pub(crate) fn get_effect_duration_s(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: ItemKey,
    a_effect: &ad::AEffectRt,
) -> Option<AttrVal> {
    let attr_id = a_effect.ae.duration_attr_id?;
    let val = calc.get_item_attr_val_full(ctx, item_key, &attr_id).ok()?;
    // Discard negative cycle time as invalid
    match val.dogma > OF(0.0) {
        true => Some(val.extra / OF(1000.0)),
        false => None,
    }
}
