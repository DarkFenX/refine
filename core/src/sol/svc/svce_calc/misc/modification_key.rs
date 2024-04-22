use crate::{
    defs::{EAttrId, SolItemId},
    sol::svc::svce_calc::{SolAttrMod, SolModOp},
};

// This is an auxiliary entity to make sure that overlapping modifications are not applied. We can
// only guess what's in actual key in EVE, and what we have here is based on observations and common
// sense. For EVE scenarios which prompt for existence of key and what's in it, see tests in
// test_similar_modifiers.py
#[derive(Eq, PartialEq, Hash)]
pub(in crate::sol::svc::svce_calc) struct SolModKey {
    pub(in crate::sol::svc::svce_calc) src_item_id: SolItemId,
    pub(in crate::sol::svc::svce_calc) src_attr_id: Option<EAttrId>,
    pub(in crate::sol::svc::svce_calc) op: SolModOp,
}
impl SolModKey {
    fn new(src_item_id: SolItemId, src_attr_id: Option<EAttrId>, op: SolModOp) -> Self {
        Self {
            src_item_id,
            src_attr_id,
            op,
        }
    }
}
impl From<&SolAttrMod> for SolModKey {
    fn from(modifier: &SolAttrMod) -> Self {
        SolModKey::new(modifier.affector_item_id, modifier.get_src_attr_id(), modifier.op)
    }
}
