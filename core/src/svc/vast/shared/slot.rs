use crate::{
    def::{Count, OF},
    rd::RAttrId,
    svc::{SvcCtx, calc::Calc},
    ud::UItemId,
};

pub(in crate::svc::vast) fn get_attr_as_count(
    ctx: SvcCtx,
    calc: &mut Calc,
    max_item_key: Option<UItemId>,
    max_attr_key: Option<RAttrId>,
) -> Option<Count> {
    calc.get_oitem_oattr_afb_oextra(ctx, max_item_key, max_attr_key, OF(0.0))
        .map(|v| v.round() as Count)
}
