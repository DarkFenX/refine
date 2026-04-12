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
        .unwrap()
        .into_value();
    val.dogma = count;
    val.extra = count;
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
        .unwrap()
        .into_value();
    AttrValInfo::new(count)
}
