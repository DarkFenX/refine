use crate::defs::EItemId;

/// Adapted modifier skill requirement.
#[derive(Debug)]
pub enum AModSrq {
    /// Targets items which skill-require item which carries the modifier.
    SelfRef,
    // Affects items which require specified skill.
    ItemId(EItemId),
}
