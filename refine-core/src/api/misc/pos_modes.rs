use crate::num::Index;

/// Defines how module is added.
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub enum MvMode {
    /// Takes a module and moves it to new position, shifting modules between original and target
    /// positions.
    Shift(Index),
    /// Swaps with specific position, which can have another module. Does not affect other modules
    /// (besides the one being moved, and the one possibly at target location).
    Swap(Index),
}

/// Defines how module is removed from a rack.
#[derive(Copy, Clone)]
pub enum RmMode {
    /// Shift all items after the item being removed to the left.
    Remove,
    /// Just free up item's place without shifting anything.
    Free,
}
