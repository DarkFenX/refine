use crate::{
    ad,
    def::{AttrVal, OF},
    misc::{AttrSpec, EffectSpec},
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItem, UItemKey},
};

pub(in crate::svc) fn get_resist_attr_id(item: &UItem, effect: &REffect) -> Option<ad::AAttrId> {
    match effect.get_resist_attr_id() {
        Some(resist_a_attr_id) => Some(resist_a_attr_id),
        None => match item.get_axt() {
            Some(item_axt) => item_axt.remote_resist_attr_id,
            None => None,
        },
    }
}

pub(crate) fn get_resist_mult_val_by_projectee_aspec(
    ctx: SvcCtx,
    calc: &mut Calc,
    projectee_aspec: &AttrSpec,
) -> Option<AttrVal> {
    let mult = calc
        .get_item_attr_val_full(ctx, projectee_aspec.item_key, &projectee_aspec.attr_id)
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
    projectee_key: UItemKey,
) -> Option<AttrVal> {
    let projector_r_effect = ctx.u_data.src.get_effect(projector_espec.effect_key);
    get_effect_resist_mult(ctx, calc, projector_espec.item_key, projector_r_effect, projectee_key)
}

pub(crate) fn get_effect_resist_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: UItemKey,
) -> Option<AttrVal> {
    let projector_item = ctx.u_data.items.get(projector_key);
    let resist_a_attr_id = get_resist_attr_id(projector_item, projector_effect)?;
    get_resist_mult_val_by_projectee_aspec(ctx, calc, &AttrSpec::new(projectee_key, resist_a_attr_id))
}
