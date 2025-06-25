use ordered_float::OrderedFloat as OF;

use crate::{
    ac, ad,
    sol::{
        ItemKey,
        svc::{
            SvcCtx,
            calc::{AttrValInfo, Calc, CalcAttrVal},
        },
    },
};

pub(in crate::sol::svc::calc) const SKILL_LVL_ATTR: ad::AAttrId = ac::attrs::SKILL_LEVEL;

pub(super) fn skill_level_postproc_fast(
    _calc: &mut Calc,
    ctx: &SvcCtx,
    item_key: ItemKey,
    mut cval: CalcAttrVal,
) -> CalcAttrVal {
    let level = ctx.uad.items.get(item_key).get_skill().unwrap().get_a_level();
    let level = OF::from(level.get_inner());
    cval.dogma = level;
    cval.extra = level;
    cval
}

pub(super) fn skill_level_postproc_info(
    _calc: &mut Calc,
    ctx: &SvcCtx,
    item_key: ItemKey,
    _info: AttrValInfo,
) -> AttrValInfo {
    let level = ctx.uad.items.get(item_key).get_skill().unwrap().get_a_level();
    let level = OF::from(level.get_inner());
    AttrValInfo::new(level)
}
