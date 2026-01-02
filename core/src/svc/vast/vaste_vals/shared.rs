use ordered_float::Float;

use crate::{
    def::{AttrVal, OF},
    rd::RAttrId,
    svc::{SvcCtx, calc::Calc},
    ud::UItemId,
    util::FLOAT_TOLERANCE,
};

pub(super) fn get_max_resource(
    ctx: SvcCtx,
    calc: &mut Calc,
    max_item_key: Option<UItemId>,
    max_attr_key: Option<RAttrId>,
) -> Option<AttrVal> {
    calc.get_item_oattr_afb_oextra(ctx, max_item_key?, max_attr_key, OF(0.0))
}

pub(super) fn is_oattr_flag_set(ctx: SvcCtx, calc: &mut Calc, item_key: UItemId, attr_key: Option<RAttrId>) -> bool {
    match attr_key {
        Some(attr_key) => is_attr_flag_set(ctx, calc, item_key, attr_key),
        None => false,
    }
}
pub(super) fn is_attr_flag_set(ctx: SvcCtx, calc: &mut Calc, item_key: UItemId, attr_key: RAttrId) -> bool {
    match calc.get_item_attr_oextra(ctx, item_key, attr_key) {
        Some(val) => val.abs() > FLOAT_TOLERANCE,
        None => false,
    }
}
