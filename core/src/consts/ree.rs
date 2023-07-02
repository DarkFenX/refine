use crate::{
    defs::{EBuffId, EItemGrpId, EItemId, Idx},
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

/// Defines how a modification will be aggregated.
///
/// When in the non-stack mode, multiple values which share the same aggregation mode and the same
/// aggregation key (the mode argument) are converted into a single value.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ModAggrMode {
    /// All modifications are applied.
    Stack,
    /// Min value will be used, from values with provided key.
    Min(EBuffId),
    /// Max value will be used, from values with provided key.
    Max(EBuffId),
}

/// Defines what kind of operation will be applied to a target attribute.
///
/// All the operations are applied in the order they are defined in this enum.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ModOp {
    /// Assigns modification value to the target item attribute before all other operations are
    /// applied.
    PreAssign,
    /// Early multiplication.
    PreMul,
    /// Early division.
    PreDiv,
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Late multiplication.
    PostMul,
    /// Late division.
    PostDiv,
    /// Late percent-alike modification, e.g. 2 + 20% = 2.4.
    PostPerc,
    /// The same as forcing attribute to modification value. When there is at least one such
    /// modification, all other modification operations are ignored.
    PostAssign,
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
