use crate::{
    consts::ModOp,
    defs::{EAttrId, SsItemId},
};

use super::AffectorSpec;

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
