use crate::defs::Idx;

/// Defines how a module is added.
pub enum SolAddMode {
    /// Add to the end of a rack.
    Append,
    /// Add to first free position of a rack.
    Equip,
    /// Add to specific position, shifting modules on this position and after it to the right.
    Insert(Idx),
    /// Add to specific position, replacing item if position is taken.
    Replace(Idx),
}

/// Defines how a module is removed from a rack.
pub enum SolRmMode {
    /// Shift all items after the item being removed to the left.
    Remove,
    /// Just free up item's place without shifting anything.
    Free,
}
