use crate::num::Index;

/// Defines how module is added.
pub enum AddMode {
    /// Add to the end of a rack.
    Append,
    /// Add to first free position of a rack.
    Equip,
    /// Add to specific position, shifting modules on this position and after it to the right.
    Insert(Index),
    /// Add to specific position, replacing item if position is taken.
    Replace(Index),
}

/// Defines how module is moved within its rack.
pub enum MvMode {
    /// Takes module from current position, shifting all modules after it to the left, and inserts
    /// at target index, shifting modules after it to the right.
    ///
    /// Index specified chooses target module position relatively other module before the
    /// shift-after-taking-module-off happens. Which means, that with initial layout \[ 1 - - 2 -\]
    /// if you move module 1 to index 4, final layout would be \[ - - 2 1 -\], not \[ - - 2 - 1\].
    Shift(Index),
    /// Swaps with specific position, which can have another module. Does not affect other modules
    /// (besides the one being moved, and the one possibly at target location).
    Swap(Index),
}

/// Defines how module is removed from a rack.
pub enum RmMode {
    /// Shift all items after the item being removed to the left.
    Remove,
    /// Just free up item's place without shifting anything.
    Free,
}
