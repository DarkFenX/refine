use crate::{
    defs::{EAttrId, SolItemId, OF},
    ec,
    sol::{
        svc::calc::{SolAttrVal, SolAttrValInfo, SolCalc},
        uad::SolUad,
    },
};

pub(super) const SKILL_LVL_ATTR: EAttrId = ec::attrs::SKILL_LEVEL;

pub(super) fn skill_level_postproc_fast(
    _: &mut SolCalc,
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
    _: &mut SolCalc,
    uad: &SolUad,
    item_id: &SolItemId,
    _: SolAttrValInfo,
) -> SolAttrValInfo {
    let level = uad.items.get_item(item_id).unwrap().get_skill().unwrap().get_level();
    let level = OF::from(level);
    SolAttrValInfo::new(level)
}
