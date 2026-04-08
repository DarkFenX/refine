use crate::ad::AItemId;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AModifierSrq {
    SelfRef,
    ItemId(AItemId),
}
