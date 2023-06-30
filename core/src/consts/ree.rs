use crate::defs::{Amount, EBuffId, EItemGrpId, EItemId, Idx};

/// Contains states which can be assigned to several entities.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum State {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl State {
    pub(crate) fn iter() -> std::slice::Iter<'static, State> {
        static STATES: [State; 5] = [
            State::Ghost,
            State::Offline,
            State::Online,
            State::Active,
            State::Overload,
        ];
        STATES.iter()
    }
}

/// Item effect operation modes.
#[derive(Copy, Clone, PartialEq)]
pub enum EffectMode {
    /// In this mode rules vary, depending on effect category:
    /// - Offline: effects from this category are run when item is in offline+
    /// state, and when they do not have fitting usage chance specified
    /// - Online: effects from this category are run when item is in online+
    /// state, and when item has runnable 'online' effect
    /// - Active: effects from this category are run when item is in active+
    /// state, and only when effect is default item effect
    /// - Overload: effects from this category
    FullCompliance,
    /// Effects in this mode are always run if item's state is high enough to
    /// run it
    StateCompliance,
    /// Effects in this mode are always running no matter what
    ForceRun,
    /// Effects in this mode are never running no matter what
    ForceStop,
}

/// Contains list of item types.
#[derive(Debug)]
pub enum ItemType {
    Booster,
    Character,
    Charge,
    Drone,
    EffectBeacon,
    FighterSquad,
    Implant,
    ModHigh,
    ModLow,
    ModMid,
    Mutaplasmid,
    Rig,
    Ship,
    Skill,
    Stance,
    Subsystem,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ModRack {
    High,
    Mid,
    Low,
}
impl std::fmt::Display for ModRack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::High => write!(f, "high"),
            Self::Mid => write!(f, "mid"),
            Self::Low => write!(f, "low"),
        }
    }
}

// enum EffectMode {
//     // In this mode rules vary, depending on effect category:
//     // - Offline: effects from this category are run when item is in offline+ state, and when
// they     // do not have fitting usage chance specified
//     // - Online: effects from this category are run when item is in online+ state, and when item
// has     // runnable 'online' effect
//     // - Active: effects from this category are run when item is in active+ state, and only when
//     // effect is default item effect
//     // - Overload: effects from this category are run when item is in overload+ state
//     FullCompliance,
//     // Effects in this mode are always run if item's state is high enough to run it
//     StateCompliance,
//     // Effects in this mode are always running no matter what
//     ForceRun,
//     // Effects in this mode are never running no matter what
//     ForceStop,
// }

/// Effect modifier build statuses.
///
/// During cache generation, the library converts modifiers of an effect into internal format.
/// Some of those modifiers might not make it through conversion process due to various reasons.
/// Variants of this enum are stored on an effect, to keep info about conversion status.
#[derive(Debug)]
pub enum ModBuildStatus {
    /// Modifiers haven't been built yet.
    Unbuilt,
    /// All modifiers failed conversion, with a failure count.
    Error(Amount),
    /// Some modifiers failed conversion, with a failure count.
    SuccessPartial(Amount),
    /// Conversion was successful.
    Success,
    /// Modifiers on an effect were customized by the library.
    Custom,
}

/// Defines which items will be affected by a modifier.
#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub enum ModAfeeFilter {
    /// Single item modified, as specified by the domain.
    Direct(ModDomain),
    /// All items belonging to the domain are affected.
    Loc(ModDomain),
    /// All items located in the domain and belonging to the group are affected.
    LocGrp(ModDomain, EItemGrpId),
    /// All items located in the domain and having specified skill requirement are affected.
    LocSrq(ModDomain, EItemId),
    /// All items belonging to the domain and having specified skill requirement are affected.
    OwnSrq(ModDomain, EItemId),
}

/// Defines domain (or scope) which is target for a modification.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum ModDomain {
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

/// Defines how a modification will be aggregated.
///
/// When in the non-stack mode, multiple values which share the same aggregation mode and the same
/// aggregation key (the mode argument) are converted into a single value.
#[derive(Debug, Copy, Clone, Hash, PartialEq)]
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
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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
