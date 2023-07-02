use crate::defs::EItemId;

/// Defines modifier skill requirement.
#[derive(Debug)]
pub enum ModSrq {
    /// Affects items which require item which carries the modifier as skill.
    SelfRef,
    // Affects items which require specific skill.
    ItemId(EItemId),
}
