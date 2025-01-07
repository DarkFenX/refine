use crate::{
    defs::{EAttrId, SolItemId, OF},
    ec,
    sol::{
        svc::{svce_calc::SolAttrVal, SolSvc},
        uad::SolUad,
    },
};

pub(super) const SKILL_LVL_ATTR: EAttrId = ec::attrs::SKILL_LEVEL;

pub(super) fn skill_level_postprocessor(
    _: &mut SolSvc,
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
