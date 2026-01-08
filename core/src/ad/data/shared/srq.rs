use crate::ad::AItemId;

#[derive(Copy, Clone)]
pub enum AModifierSrq {
    SelfRef,
    ItemId(AItemId),
}
