/// Item effect operation modes.
#[derive(Copy, Clone, PartialEq)]
pub enum EffectMode {
    /// In this mode rules vary, depending on effect category:
    /// - Offline: effects from this category are run when item is in offline+ state, and when they
    ///   do not have fitting usage chance specified
    /// - Online: effects from this category are run when item is in online+ state, and when item
    ///   has runnable 'online' effect
    /// - Active: effects from this category are run when item is in active+ state, and only when
    ///   effect is default item effect
    /// - Overload: effects from this category
    FullCompliance,
    /// Effects in this mode are always run if item's state is high enough to run it
    StateCompliance,
    /// Effects in this mode are always running no matter what
    ForceRun,
    /// Effects in this mode are never running no matter what
    ForceStop,
}
