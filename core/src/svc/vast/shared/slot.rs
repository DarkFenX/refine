use crate::{
    ad,
    def::{Count, ItemKey},
    svc::{SvcCtx, calc::Calc},
};

pub(in crate::svc::vast) fn get_attr_as_count(
    ctx: SvcCtx,
    calc: &mut Calc,
    max_item_key: Option<ItemKey>,
    max_a_attr_id: &ad::AAttrId,
) -> Option<Count> {
    calc.get_item_attr_val_extra_opt_opt(ctx, max_item_key, max_a_attr_id)
        .map(|v| v.round() as Count)
}
