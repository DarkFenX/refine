use crate::{
    defs::{EItemGrpId, EItemId, Idx},
    shr::ModDomain,
};

/// Defines which items will be affected by a modifier.
#[derive(Debug)]
pub enum ModAfeeFilter {
    /// Single item modified, as specified by the domain.
    Direct(ModDomain),
    /// All items belonging to the domain are affected.
    Loc(ModDomain),
    /// All items located in the domain and belonging to the group are affected.
    LocGrp(ModDomain, EItemGrpId),
    /// All items located in the domain and having specified skill requirement are affected.
    LocSrq(ModDomain, ModSrq),
    /// All items belonging to the domain and having specified skill requirement are affected.
    OwnSrq(ModDomain, ModSrq),
}

/// Defines modifier skill requirement.
#[derive(Debug)]
pub enum ModSrq {
    /// Affects items which require item which carries the modifier as skill.
    SelfRef,
    // Affects items which require specific skill.
    ItemId(EItemId),
}

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

/// Defines how an item is added to an ordered container.
pub enum OrdAddMode {
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
pub enum OrdRmMode {
    /// Shift all items after the item being removed to the left.
    Remove,
    /// Just free up item's place without shifting anything.
    Free,
}
