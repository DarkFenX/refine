use crate::{
    defs::{AttrVal, EAttrId, SolItemId},
    ec,
    sol::{
        svc::{svce_calc::SolAttrVal, SolSvcs},
        SolView,
    },
};

pub(super) const SKILL_LVL_ATTR: EAttrId = ec::attrs::SKILL_LEVEL;

pub(super) fn skill_level_postprocessor(
    _: &mut SolSvcs,
    sol_view: &SolView,
    item_id: &SolItemId,
    mut val: SolAttrVal,
) -> SolAttrVal {
    let level = sol_view
        .items
        .get_item(item_id)
        .unwrap()
        .get_skill()
        .unwrap()
        .get_level();
    val.dogma = level as AttrVal;
    val.extra = level as AttrVal;
    val
}
