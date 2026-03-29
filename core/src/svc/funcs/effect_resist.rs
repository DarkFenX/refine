use crate::{
    ad::{AAttrId, AEveAttrId},
    misc::AttrSpec,
    num::{PValue, Value},
    rd::{RAttrId, REffect},
    svc::{SvcCtx, calc::Calc},
    ud::{UItem, UItemId},
};

pub(in crate::svc) fn get_effect_default_resist_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    projector_effect: &REffect,
    projectee_uid: UItemId,
) -> Option<PValue> {
    let projector_item = ctx.u_data.items.get(projector_uid);
    let attr_rid = get_resist_attr_rid(projector_item, projector_effect)?;
    get_resist_mult(ctx, calc, &AttrSpec::new(projectee_uid, attr_rid))
}

pub(in crate::svc) fn get_referenced_resist_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_aspec: &AttrSpec,
    projectee_uid: UItemId,
) -> Option<PValue> {
    let attr_id_raw = calc.get_item_attr_odogma(ctx, projector_aspec.item_uid, projector_aspec.attr_rid)?;
    let attr_aid = AAttrId::Eve(AEveAttrId::from_f64_rounded(attr_id_raw.into_f64()));
    let attr_rid = ctx.u_data.src.get_attr_rid_by_aid(&attr_aid)?;
    get_resist_mult(ctx, calc, &AttrSpec::new(projectee_uid, attr_rid))
}

pub(in crate::svc) fn get_resist_attr_rid(item: &UItem, effect: &REffect) -> Option<RAttrId> {
    match effect.resist_attr_rid {
        Some(resist_attr_rid) => Some(resist_attr_rid),
        None => match item.get_axt() {
            Some(item_axt) => item_axt.remote_resist_attr_rid,
            None => None,
        },
    }
}

pub(in crate::svc) fn get_resist_mult(ctx: SvcCtx, calc: &mut Calc, projectee_aspec: &AttrSpec) -> Option<PValue> {
    let mult = calc.get_item_attr_odogma(ctx, projectee_aspec.item_uid, projectee_aspec.attr_rid)?;
    Some(match mult <= Value::from_f64(0.0001) {
        true => PValue::ZERO,
        false => PValue::from_f64_unchecked(mult.into_f64()),
    })
}
