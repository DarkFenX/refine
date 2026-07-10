use crate::{
    num::{Count, Value},
    rd::RAttrId,
    svc::{SvcCtx, calc::Calc},
    ud::UItemId,
};

pub(in crate::svc::vast) fn get_attr_as_count(
    ctx: SvcCtx,
    calc: &mut Calc,
    max_item_uid: Option<UItemId>,
    max_attr_rid: Option<RAttrId>,
) -> Option<Count> {
    calc.get_oitem_oattr_afb_oextra(ctx, max_item_uid, max_attr_rid, Value::ZERO)
        .map(Count::from_value_rounded)
}
