use crate::ad::AItemId;

#[derive(Copy, Clone)]
pub enum AModifierSrq {
    SelfRef,
    TypeId(AItemId),
}
