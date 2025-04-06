use ordered_float::OrderedFloat as OF;

use crate::{
    ac, ad,
    sol::{
        ItemId,
        svc::calc::{AttrValInfo, Calc, CalcAttrVal},
        uad::Uad,
    },
};

pub(in crate::sol::svc::calc) const SKILL_LVL_ATTR: ad::AAttrId = ac::attrs::SKILL_LEVEL;

pub(super) fn skill_level_postproc_fast(
    _calc: &mut Calc,
    uad: &Uad,
    item_id: &ItemId,
    mut cval: CalcAttrVal,
) -> CalcAttrVal {
    let level = uad.items.get_by_id(item_id).unwrap().get_skill().unwrap().get_a_level();
    let level = OF::from(level);
    cval.dogma = level;
    cval.extra = level;
    cval
}

pub(super) fn skill_level_postproc_info(
    _calc: &mut Calc,
    uad: &Uad,
    item_id: &ItemId,
    _info: AttrValInfo,
) -> AttrValInfo {
    let level = uad.items.get_by_id(item_id).unwrap().get_skill().unwrap().get_a_level();
    let level = OF::from(level);
    AttrValInfo::new(level)
}
