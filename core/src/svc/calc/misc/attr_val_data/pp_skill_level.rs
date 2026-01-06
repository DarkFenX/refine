use crate::{
    misc::Value,
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVals},
    },
    ud::UItemId,
};

pub(super) fn skill_level_postproc_fast(
    _calc: &mut Calc,
    ctx: SvcCtx,
    item_uid: UItemId,
    mut cval: CalcAttrVals,
) -> CalcAttrVals {
    let level = ctx.u_data.items.get(item_uid).dc_skill().unwrap().get_level();
    let level = Value::from(level);
    cval.dogma = level;
    cval.extra = level;
    cval
}

pub(super) fn skill_level_postproc_info(
    _calc: &mut Calc,
    ctx: SvcCtx,
    item_uid: UItemId,
    _info: AttrValInfo,
) -> AttrValInfo {
    let level = ctx.u_data.items.get(item_uid).dc_skill().unwrap().get_level();
    let level = Value::from(level);
    AttrValInfo::new(level)
}
