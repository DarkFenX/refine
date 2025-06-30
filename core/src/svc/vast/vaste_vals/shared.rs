use crate::{
    ad,
    def::{AttrVal, ItemKey, OF},
    svc::{SvcCtx, calc::Calc},
};

pub(super) fn get_max_resource(
    ctx: &SvcCtx,
    calc: &mut Calc,
    max_item_key: Option<ItemKey>,
    max_a_attr_id: &ad::AAttrId,
) -> Option<AttrVal> {
    calc.get_item_attr_val_extra_opt(ctx, max_item_key, max_a_attr_id)
}

pub(super) fn is_flag_set(ctx: &SvcCtx, calc: &mut Calc, item_key: ItemKey, a_attr_id: &ad::AAttrId) -> bool {
    match calc.get_item_attr_val_extra(ctx, item_key, a_attr_id) {
        Some(val) => val != OF(0.0),
        None => false,
    }
}
