use crate::{
    ad,
    def::{AttrVal, ItemKey, OF},
    misc::{AttrSpec, EffectSpec},
    svc::{SvcCtx, calc::Calc},
    uad::UadItem,
};

pub(in crate::svc) fn get_resist_a_attr_id(item: &UadItem, a_effect: &ad::AEffectRt) -> Option<ad::AAttrId> {
    match a_effect.ae.resist_attr_id {
        Some(resist_a_attr_id) => Some(resist_a_attr_id),
        None => match item.get_a_xt() {
            Some(a_item_xt) => a_item_xt.remote_resist_attr_id,
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
    projectee_key: ItemKey,
) -> Option<AttrVal> {
    let projector_a_effect = ctx.uad.src.get_a_effect(&projector_espec.a_effect_id)?;
    get_effect_resist_mult(ctx, calc, projector_espec.item_key, projector_a_effect, projectee_key)
}

pub(crate) fn get_effect_resist_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: ItemKey,
    projector_a_effect: &ad::AEffectRt,
    projectee_key: ItemKey,
) -> Option<AttrVal> {
    let projector_item = ctx.uad.items.get(projector_key);
    let resist_a_attr_id = get_resist_a_attr_id(projector_item, projector_a_effect)?;
    get_resist_mult_val_by_projectee_aspec(ctx, calc, &AttrSpec::new(projectee_key, resist_a_attr_id))
}
