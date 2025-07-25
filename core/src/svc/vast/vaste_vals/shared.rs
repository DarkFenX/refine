use crate::{
    ad,
    def::{AttrVal, OF},
    svc::{SvcCtx, calc::Calc},
    ud::UItemKey,
};

pub(super) fn get_max_resource(
    ctx: SvcCtx,
    calc: &mut Calc,
    max_item_key: Option<UItemKey>,
    max_a_attr_id: &ad::AAttrId,
) -> Option<AttrVal> {
    calc.get_item_attr_val_extra_opt_opt(ctx, max_item_key, max_a_attr_id)
}

pub(super) fn is_flag_set(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey, a_attr_id: &ad::AAttrId) -> bool {
    match calc.get_item_attr_val_extra_opt(ctx, item_key, a_attr_id) {
        Some(val) => val != OF(0.0),
        None => false,
    }
}
