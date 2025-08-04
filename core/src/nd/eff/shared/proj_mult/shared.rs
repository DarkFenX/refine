use crate::{
    ad::AAttrId,
    def::{AttrVal, OF},
    svc::{SvcCtx, calc::Calc},
    ud::UItemKey,
};

pub(super) fn get_range(ctx: SvcCtx, calc: &mut Calc, affector_key: UItemKey, attr_id: Option<AAttrId>) -> AttrVal {
    match attr_id {
        Some(attr_id) => match calc.get_item_attr_val_full(ctx, affector_key, &attr_id) {
            Ok(val) => val.extra,
            _ => OF(0.0),
        },
        None => OF(0.0),
    }
}
