use crate::{
    def::OF,
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVal},
    },
    ud::UItemKey,
};

pub(super) fn skill_level_postproc_fast(
    _calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemKey,
    mut cval: CalcAttrVal,
) -> CalcAttrVal {
    let level = ctx.u_data.items.get(item_key).dc_skill().unwrap().get_level();
    let level = OF::from(level.get_inner());
    cval.dogma = level;
    cval.extra = level;
    cval
}

pub(super) fn skill_level_postproc_info(
    _calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemKey,
    _info: AttrValInfo,
) -> AttrValInfo {
    let level = ctx.u_data.items.get(item_key).dc_skill().unwrap().get_level();
    let level = OF::from(level.get_inner());
    AttrValInfo::new(level)
}
