use crate::{
    consts::ModOp,
    defs::{EAttrId, SsItemId},
};

use super::AffectorSpec;

// This is an auxiliary entity to make sure that overlapping modifications are
// not applied. We can only guess what's in actual key in EVE, and what we have
// here is based on observations and common sense. For EVE scenarios which
// prompt for existence of key and what's in it, see tests in
// "test_similar_modifiers.py"
#[derive(Hash, Eq, PartialEq)]
pub(in crate::ss::svc::calc) struct ModKey {
    pub(in crate::ss::svc::calc) afor_item_id: SsItemId,
    pub(in crate::ss::svc::calc) afor_attr_id: EAttrId,
    pub(in crate::ss::svc::calc) op: ModOp,
}
impl ModKey {
    fn new(afor_item_id: SsItemId, afor_attr_id: EAttrId, op: ModOp) -> Self {
        Self {
            afor_item_id,
            afor_attr_id,
            op,
        }
    }
}
impl From<&AffectorSpec> for ModKey {
    fn from(affector_spec: &AffectorSpec) -> Self {
        ModKey::new(
            affector_spec.item_id,
            affector_spec.modifier.afor_attr_id,
            affector_spec.modifier.op,
        )
    }
}
