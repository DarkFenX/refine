use crate::defs::Idx;

/// Defines how an item is added to an ordered container.
pub enum SolOrdAddMode {
    /// Add to the end of container.
    Append,
    /// Add to first free position of container.
    Equip,
    /// Add to specific position, shifting modules on this position and after it to the right.
    Insert(Idx),
    /// Add to specific position, replacing item on it if 2nd argument is true.
    Place(Idx, bool),
}

/// Defines how an item is removed from an ordered container.
pub enum SolOrdRmMode {
    /// Shift all items after the item being removed to the left.
    Remove,
    /// Just free up item's place without shifting anything.
    Free,
}
