use crate::{
    svc::{
        SvcCtx,
        calc::{AttrValInfo, CalcAttrVals},
    },
    ud::UItemId,
};

pub(super) fn fighter_count_postproc_fast(ctx: SvcCtx, item_uid: UItemId, mut val: CalcAttrVals) -> CalcAttrVals {
    let count = ctx
        .u_data
        .items
        .get(item_uid)
        .dc_fighter()
        .unwrap()
        .get_count()
        .unwrap();
    let current_count = count.current.into_value();
    val.dogma = current_count;
    val.extra = current_count;
    val
}

pub(super) fn fighter_count_postproc_info(ctx: SvcCtx, item_uid: UItemId) -> AttrValInfo {
    let count = ctx
        .u_data
        .items
        .get(item_uid)
        .dc_fighter()
        .unwrap()
        .get_count()
        .unwrap();
    let current_count = count.current.into_value();
    AttrValInfo::new(current_count)
}
