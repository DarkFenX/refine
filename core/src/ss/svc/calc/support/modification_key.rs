use crate::{
    defs::{EAttrId, SsItemId},
    shr::ModOp,
};

use super::SsAttrMod;

// This is an auxiliary entity to make sure that overlapping modifications are
// not applied. We can only guess what's in actual key in EVE, and what we have
// here is based on observations and common sense. For EVE scenarios which
// prompt for existence of key and what's in it, see tests in
// "test_similar_modifiers.py"
#[derive(Eq, PartialEq, Hash)]
pub(in crate::ss::svc::calc) struct ModKey {
    pub(in crate::ss::svc::calc) src_item_id: SsItemId,
    pub(in crate::ss::svc::calc) src_attr_id: EAttrId,
    pub(in crate::ss::svc::calc) op: ModOp,
}
impl ModKey {
    fn new(src_item_id: SsItemId, src_attr_id: EAttrId, op: ModOp) -> Self {
        Self {
            src_item_id,
            src_attr_id,
            op,
        }
    }
}
impl From<&SsAttrMod> for ModKey {
    fn from(modifier: &SsAttrMod) -> Self {
        ModKey::new(modifier.src_item_id, modifier.src_attr_id, modifier.op)
    }
}
