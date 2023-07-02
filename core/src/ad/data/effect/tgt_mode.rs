/// Defines how effects like fighter abilities are targeted.
#[derive(Debug)]
pub enum TgtMode {
    /// No target needed.
    None,
    /// Specific item is needed for the effect to activate.
    Item,
    /// Specific point in space is needed for the effect to activate.
    Point,
}
