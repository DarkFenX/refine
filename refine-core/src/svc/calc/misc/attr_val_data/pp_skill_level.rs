use crate::{
    svc::{
        SvcCtx,
        calc::{AttrValInfo, CalcAttrVals},
    },
    ud::UItemId,
};

pub(super) fn skill_level_postproc_fast(ctx: SvcCtx, item_uid: UItemId, mut cval: CalcAttrVals) -> CalcAttrVals {
    let level = ctx
        .u_data
        .items
        .get(item_uid)
        .dc_skill()
        .unwrap()
        .get_level()
        .into_value();
    cval.dogma = level;
    cval.extra = level;
    cval
}

pub(super) fn skill_level_postproc_info(ctx: SvcCtx, item_uid: UItemId) -> AttrValInfo {
    let level = ctx
        .u_data
        .items
        .get(item_uid)
        .dc_skill()
        .unwrap()
        .get_level()
        .into_value();
    AttrValInfo::new(level)
}
