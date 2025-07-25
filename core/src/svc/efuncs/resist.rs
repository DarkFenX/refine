use crate::{
    ad,
    def::{AttrVal, OF},
    misc::{AttrSpec, EffectSpec},
    rd,
    svc::{SvcCtx, calc::Calc},
    uad::{UadItem, UadItemKey},
};

pub(in crate::svc) fn get_resist_a_attr_id(item: &UadItem, r_effect: &rd::REffect) -> Option<ad::AAttrId> {
    match r_effect.get_resist_attr_id() {
        Some(resist_a_attr_id) => Some(resist_a_attr_id),
        None => match item.get_r_axt() {
            Some(r_item_axt) => r_item_axt.remote_resist_attr_id,
            None => None,
        },
    }
}

pub(in crate::svc) fn get_resist_mult_val_by_projectee_aspec(
    ctx: SvcCtx,
    calc: &mut Calc,
    projectee_aspec: &AttrSpec,
) -> Option<AttrVal> {
    let mult = calc
        .get_item_attr_val_full(ctx, projectee_aspec.item_key, &projectee_aspec.a_attr_id)
        .ok()?
        .dogma;
    Some(match mult.abs() <= 0.0001 {
        true => OF(0.0),
        false => mult,
    })
}

pub(crate) fn get_espec_resist_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_espec: &EffectSpec,
    projectee_key: UadItemKey,
) -> Option<AttrVal> {
    let projector_r_effect = ctx.uad.src.get_r_effect(&projector_espec.a_effect_id)?;
    get_effect_resist_mult(ctx, calc, projector_espec.item_key, projector_r_effect, projectee_key)
}

pub(crate) fn get_effect_resist_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UadItemKey,
    projector_r_effect: &rd::REffect,
    projectee_key: UadItemKey,
) -> Option<AttrVal> {
    let projector_item = ctx.uad.items.get(projector_key);
    let resist_a_attr_id = get_resist_a_attr_id(projector_item, projector_r_effect)?;
    get_resist_mult_val_by_projectee_aspec(ctx, calc, &AttrSpec::new(projectee_key, resist_a_attr_id))
}
