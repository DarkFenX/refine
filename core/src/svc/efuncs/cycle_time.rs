use crate::{
    ad,
    def::{AttrVal, ItemKey, OF},
    misc::EffectSpec,
    svc::{SvcCtx, calc::Calc},
};

pub(crate) fn get_espec_cycle_time(ctx: SvcCtx, calc: &mut Calc, espec: EffectSpec) -> Option<AttrVal> {
    let a_effect = ctx.uad.src.get_a_effect(&espec.a_effect_id)?;
    get_effect_cycle_time(ctx, calc, espec.item_key, a_effect)
}

pub(in crate::svc) fn get_effect_cycle_time(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: ItemKey,
    a_effect: &ad::AEffectRt,
) -> Option<AttrVal> {
    let attr_id = a_effect.ae.duration_attr_id?;
    let val = calc.get_item_attr_val_full(ctx, item_key, &attr_id).ok()?;
    Some(val.dogma / OF(1000.0))
}

pub(in crate::svc) fn has_cycle_time(a_effect: &ad::AEffectRt) -> bool {
    a_effect.ae.duration_attr_id.is_some()
}
