/// Defines domain (or scope) which is target for a modification.
#[derive(PartialEq)]
pub enum AModDomain {
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
}
