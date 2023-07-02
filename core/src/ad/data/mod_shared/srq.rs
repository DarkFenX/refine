use crate::defs::EItemId;

/// Adapted modifier skill requirement.
#[derive(Debug)]
pub enum AModSrq {
    /// Affects items which require item which carries the modifier as skill.
    SelfRef,
    // Affects items which require specified skill.
    ItemId(EItemId),
}
