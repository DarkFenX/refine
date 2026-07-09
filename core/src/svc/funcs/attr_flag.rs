use crate::{
    num::PValue,
    rd::RAttrId,
    svc::{SvcCtx, calc::Calc},
    ud::UItemId,
};

pub(in crate::svc) fn is_oattr_flag_set(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    attr_rid: Option<RAttrId>,
) -> Option<bool> {
    is_attr_flag_set(ctx, calc, item_uid, attr_rid?)
}

pub(in crate::svc) fn is_attr_flag_set(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    attr_rid: RAttrId,
) -> Option<bool> {
    calc.get_item_attr_oextra(ctx, item_uid, attr_rid)
        .map(|v| v.abs() > PValue::FLOAT_TOLERANCE)
}
