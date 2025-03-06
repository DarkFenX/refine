use crate::{
    defs::{EAttrId, OF, SolItemId},
    ec,
    sol::{
        svc::calc::{SolAttrVal, SolAttrValInfo, SolCalc},
        uad::SolUad,
    },
};

pub(in crate::sol::svc::calc) const SKILL_LVL_ATTR: EAttrId = ec::attrs::SKILL_LEVEL;

pub(super) fn skill_level_postproc_fast(
    _calc: &mut SolCalc,
    uad: &SolUad,
    item_id: &SolItemId,
    mut val: SolAttrVal,
) -> SolAttrVal {
    let level = uad.items.get_item(item_id).unwrap().get_skill().unwrap().get_level();
    let level = OF::from(level);
    val.dogma = level;
    val.extra = level;
    val
}

pub(super) fn skill_level_postproc_info(
    _calc: &mut SolCalc,
    uad: &SolUad,
    item_id: &SolItemId,
    _info: SolAttrValInfo,
) -> SolAttrValInfo {
    let level = uad.items.get_item(item_id).unwrap().get_skill().unwrap().get_level();
    let level = OF::from(level);
    SolAttrValInfo::new(level)
}
