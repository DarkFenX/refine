use crate::{
    rd::RAttrId,
    svc::calc::{CalcOp, CtxModifier},
    ud::UItemId,
};

// This is an auxiliary entity to make sure that overlapping modifications are not applied. We can
// only guess what's in actual key in EVE, and what we have here is based on observations and common
// sense. For EVE scenarios which prompt for existence of key and what's in it, see tests in
// test_similar_modifiers.py
#[derive(Eq, PartialEq, Hash)]
pub(in crate::svc::calc) struct CalcModificationKey {
    pub(in crate::svc::calc) affector_key: UItemId,
    pub(in crate::svc::calc) affector_attr_key: Option<RAttrId>,
    pub(in crate::svc::calc) op: CalcOp,
}
impl From<&CtxModifier> for CalcModificationKey {
    fn from(cmod: &CtxModifier) -> Self {
        CalcModificationKey {
            affector_key: cmod.raw.affector_espec.item_key,
            affector_attr_key: cmod.raw.get_affector_attr_key(),
            op: cmod.raw.op,
        }
    }
}
