use crate::{
    num::Value,
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
