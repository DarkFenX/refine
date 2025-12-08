use crate::ad::AItemId;

/// Adapted modifier skill requirement.
#[derive(Copy, Clone)]
pub enum AModifierSrq {
    /// Affects items which skill-require item which carries the modifier.
    SelfRef,
    /// Affects items which require specified skill.
    TypeId(AItemId),
}
