use crate::{
    defs::{EAttrId, SolItemId},
    sol::svc::svce_calc::{SolCtxModifier, SolOp},
};

// This is an auxiliary entity to make sure that overlapping modifications are not applied. We can
// only guess what's in actual key in EVE, and what we have here is based on observations and common
// sense. For EVE scenarios which prompt for existence of key and what's in it, see tests in
// test_similar_modifiers.py
#[derive(Eq, PartialEq, Hash)]
pub(in crate::sol::svc::svce_calc) struct SolModificationKey {
    pub(in crate::sol::svc::svce_calc) affector_item_id: SolItemId,
    pub(in crate::sol::svc::svce_calc) affector_attr_id: Option<EAttrId>,
    pub(in crate::sol::svc::svce_calc) op: SolOp,
}
impl SolModificationKey {
    fn new(affector_item_id: SolItemId, affector_attr_id: Option<EAttrId>, op: SolOp) -> Self {
        Self {
            affector_item_id,
            affector_attr_id,
            op,
        }
    }
}
impl From<&SolCtxModifier> for SolModificationKey {
    fn from(modifier: &SolCtxModifier) -> Self {
        SolModificationKey::new(
            modifier.raw.affector_item_id,
            modifier.raw.get_affector_attr_id(),
            modifier.raw.op,
        )
    }
}
