use crate::{
    defs::{EAttrId, SsItemId},
    ss::svc::svce_calc::{SsAttrMod, SsModOp},
};

// This is an auxiliary entity to make sure that overlapping modifications are not applied. We can
// only guess what's in actual key in EVE, and what we have here is based on observations and common
// sense. For EVE scenarios which prompt for existence of key and what's in it, see tests in
// test_similar_modifiers.py
#[derive(Eq, PartialEq, Hash)]
pub(in crate::ss::svc::svce_calc) struct SsModKey {
    pub(in crate::ss::svc::svce_calc) src_item_id: SsItemId,
    pub(in crate::ss::svc::svce_calc) src_attr_id: Option<EAttrId>,
    pub(in crate::ss::svc::svce_calc) op: SsModOp,
}
impl SsModKey {
    fn new(src_item_id: SsItemId, src_attr_id: Option<EAttrId>, op: SsModOp) -> Self {
        Self {
            src_item_id,
            src_attr_id,
            op,
        }
    }
}
impl From<&SsAttrMod> for SsModKey {
    fn from(modifier: &SsAttrMod) -> Self {
        SsModKey::new(modifier.affector_item_id, modifier.get_src_attr_id(), modifier.op)
    }
}
