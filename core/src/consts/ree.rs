use crate::defines::ReeInt;

/// Contains states which can be assigned to several entities.
#[derive(Debug)]
pub enum State {
    Offline,
    Online,
    Active,
    Overload,
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
#[derive(Debug)]
pub enum ModBuildStatus {
    Unbuilt,
    Error,
    SuccessPartial,
    Success,
    Custom,
}

/// Defines which items will be affected by a modifier.
#[derive(Debug)]
pub enum ModAfeeFilter {
    /// Single item modified, as specified by the domain.
    Direct(ModDomain),
    /// All items belonging to the domain are affected.
    Loc(ModDomain),
    /// All items located in the domain and belonging to the group are affected.
    LocGrp(ModDomain, ReeInt),
    /// All items located in the domain and having specified skill requirement are affected.
    LocSrq(ModDomain, ReeInt),
    /// All items belonging to the domain and having specified skill requirement are affected.
    OwnSrq(ModDomain, ReeInt),
}

#[derive(Debug)]
pub enum ModDomain {
    Ship,
    Structure,
    Char,
    Item,
    Other,
}

/// Defines how a modification will be aggregated.
///
/// When in the non-stack mode, multiple values which share the same aggregation mode and the same
/// aggregation key (the mode argument) are converted into a single value.
#[derive(Debug)]
pub enum ModAggrMode {
    /// No aggregation.
    Stack,
    /// Min value will be used, from values with provided key.
    Min(ReeInt),
    /// Max value will be used, from values with provided key.
    Max(ReeInt),
}

/// Defines what kind of operation will be applied to a target attribute.
///
/// All the operations are applied in the order they are defined in this enum.
#[derive(Debug)]
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
    /// Specific item is needed for the ability to activate.
    Item,
    /// Specific point in space is needed for the ability to activate.
    Point,
}
