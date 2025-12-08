use crate::{
    def::{AttrVal, OF},
    misc::AttrSpec,
    rd::{RAttrKey, REffect},
    svc::{SvcCtx, calc::Calc},
    ud::{UItem, UItemKey},
};

pub(in crate::svc) fn get_resist_attr_key(item: &UItem, effect: &REffect) -> Option<RAttrKey> {
    match effect.resist_attr_key {
        Some(resist_attr_key) => Some(resist_attr_key),
        None => match item.get_axt() {
            Some(item_axt) => item_axt.remote_resist_attr_key,
            None => None,
        },
    }
}

pub(crate) fn get_resist_mult_by_projectee_aspec(
    ctx: SvcCtx,
    calc: &mut Calc,
    projectee_aspec: &AttrSpec,
) -> Option<AttrVal> {
    let mult = calc.get_item_attr_odogma(ctx, projectee_aspec.item_key, projectee_aspec.attr_key)?;
    Some(match mult.abs() <= 0.0001 {
        true => OF(0.0),
        false => mult,
    })
}

pub(crate) fn get_effect_resist_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: UItemKey,
) -> Option<AttrVal> {
    let projector_item = ctx.u_data.items.get(projector_key);
    let resist_attr_key = get_resist_attr_key(projector_item, projector_effect)?;
    get_resist_mult_by_projectee_aspec(ctx, calc, &AttrSpec::new(projectee_key, resist_attr_key))
}
