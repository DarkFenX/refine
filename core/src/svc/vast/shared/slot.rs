use crate::{
    def::Count,
    rd::RAttrKey,
    svc::{SvcCtx, calc::Calc},
    ud::UItemKey,
};

pub(in crate::svc::vast) fn get_attr_as_count(
    ctx: SvcCtx,
    calc: &mut Calc,
    max_item_key: Option<UItemKey>,
    max_attr_key: Option<RAttrKey>,
) -> Option<Count> {
    // Return None only when there is no item
    let max_item_key = max_item_key?;
    let max_attr_key = match max_attr_key {
        Some(max_attr_key) => max_attr_key,
        None => return Some(0),
    };
    match calc.get_item_attr_rextra(ctx, max_item_key, max_attr_key) {
        Ok(val) => Some(val.round() as Count),
        Err(_) => Some(0),
    }
}
