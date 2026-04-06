use crate::{
    num::{PValue, Value},
    rd::RAttrId,
    svc::{SvcCtx, calc::Calc},
    ud::UItemId,
};

pub(super) fn get_max_resource(
    ctx: SvcCtx,
    calc: &mut Calc,
    max_item_uid: Option<UItemId>,
    max_attr_rid: Option<RAttrId>,
) -> Option<Value> {
    calc.get_item_oattr_afb_oextra(ctx, max_item_uid?, max_attr_rid, Value::ZERO)
}

pub(super) fn is_oattr_flag_set(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    attr_rid: Option<RAttrId>,
) -> Option<bool> {
    is_attr_flag_set(ctx, calc, item_uid, attr_rid?)
}
pub(super) fn is_attr_flag_set(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, attr_rid: RAttrId) -> Option<bool> {
    calc.get_item_attr_oextra(ctx, item_uid, attr_rid)
        .map(|v| v.abs() > PValue::FLOAT_TOLERANCE)
}
