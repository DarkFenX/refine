/// Defines domain which is affected by a modification.
pub enum AEffectDomain {
    /// Ship or items belonging to it.
    Ship,
    /// Structure or items belonging to it.
    Structure,
    /// Character or items owned by it.
    Char,
    /// Specific single item.
    Item,
    /// Charge for module, module for charge.
    Other,
    /// Affects target or items belonging to it.
    Target,
}
