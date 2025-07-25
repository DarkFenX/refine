use crate::{
    ad,
    def::{AttrVal, OF},
    svc::{SvcCtx, calc::Calc},
    ud::UItemKey,
};

pub(super) fn get_range(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: UItemKey,
    a_attr_id: Option<ad::AAttrId>,
) -> AttrVal {
    match a_attr_id {
        Some(a_attr_id) => match calc.get_item_attr_val_full(ctx, affector_key, &a_attr_id) {
            Ok(val) => val.dogma,
            _ => OF(0.0),
        },
        None => OF(0.0),
    }
}
