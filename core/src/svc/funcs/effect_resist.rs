use crate::{
    misc::{AttrSpec, PValue, Value},
    rd::{RAttrId, REffect},
    svc::{SvcCtx, calc::Calc},
    ud::{UItem, UItemId},
};

pub(in crate::svc) fn get_resist_attr_rid(item: &UItem, effect: &REffect) -> Option<RAttrId> {
    match effect.resist_attr_rid {
        Some(resist_attr_rid) => Some(resist_attr_rid),
        None => match item.get_axt() {
            Some(item_axt) => item_axt.remote_resist_attr_rid,
            None => None,
        },
    }
}

pub(crate) fn get_resist_mult_by_projectee_aspec(
    ctx: SvcCtx,
    calc: &mut Calc,
    projectee_aspec: &AttrSpec,
) -> Option<PValue> {
    let mult = calc
        .get_item_attr_rfull(ctx, projectee_aspec.item_uid, projectee_aspec.attr_rid)
        .ok()?
        .dogma;
    Some(match mult <= Value::from_f64(0.0001) {
        true => PValue::ZERO,
        false => PValue::new_unchecked(mult.into_f64()),
    })
}

pub(crate) fn get_effect_resist_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    projector_effect: &REffect,
    projectee_uid: UItemId,
) -> Option<PValue> {
    let projector_item = ctx.u_data.items.get(projector_uid);
    let resist_attr_rid = get_resist_attr_rid(projector_item, projector_effect)?;
    get_resist_mult_by_projectee_aspec(ctx, calc, &AttrSpec::new(projectee_uid, resist_attr_rid))
}
