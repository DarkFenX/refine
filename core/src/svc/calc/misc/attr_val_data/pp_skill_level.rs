use crate::{
    def::AttrVal,
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVals},
    },
    ud::UItemId,
};

pub(super) fn skill_level_postproc_fast(
    _calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemId,
    mut cval: CalcAttrVals,
) -> CalcAttrVals {
    let level = ctx.u_data.items.get(item_key).dc_skill().unwrap().get_level();
    let level = AttrVal::from(level.get_inner());
    cval.dogma = level;
    cval.extra = level;
    cval
}

pub(super) fn skill_level_postproc_info(
    _calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemId,
    _info: AttrValInfo,
) -> AttrValInfo {
    let level = ctx.u_data.items.get(item_key).dc_skill().unwrap().get_level();
    let level = AttrVal::from(level.get_inner());
    AttrValInfo::new(level)
}
