/// Adapted modifier skill requirement.
pub enum AModifierSrq {
    /// Affects items which skill-require item which carries the modifier.
    SelfRef,
    /// Affects items which require specified skill.
    ItemId(EItemId),
}
