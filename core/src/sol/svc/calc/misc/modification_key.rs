use crate::{
    ad,
    sol::{
        ItemKey,
        svc::calc::{CtxModifier, Op},
    },
};

// This is an auxiliary entity to make sure that overlapping modifications are not applied. We can
// only guess what's in actual key in EVE, and what we have here is based on observations and common
// sense. For EVE scenarios which prompt for existence of key and what's in it, see tests in
// test_similar_modifiers.py
#[derive(Eq, PartialEq, Hash)]
pub(in crate::sol::svc::calc) struct ModificationKey {
    pub(in crate::sol::svc::calc) affector_item_key: ItemKey,
    pub(in crate::sol::svc::calc) affector_a_attr_id: Option<ad::AAttrId>,
    pub(in crate::sol::svc::calc) op: Op,
}
impl From<&CtxModifier> for ModificationKey {
    fn from(modifier: &CtxModifier) -> Self {
        ModificationKey {
            affector_item_key: modifier.raw.affector_item_key,
            affector_a_attr_id: modifier.raw.get_affector_a_attr_id(),
            op: modifier.raw.op,
        }
    }
}
