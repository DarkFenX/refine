/// Item effect operation modes.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Copy, Clone, PartialEq)]
pub enum EffectMode {
    /// In this mode rules vary, depending on effect category:
    /// - Offline: effects from this category are run when item is in offline+ state, and when they
    ///   do not have fitting usage chance specified
    /// - Online: effects from this category are run when item is in online+ state, and when item
    ///   has runnable 'online' effect
    /// - Active: effects from this category are run when item is in active+ state, and if item is
    ///   drone, or if it is default effect of item effect is default item effect
    /// - Overload: effects from this category
    #[cfg_attr(feature = "serde", serde(rename = "full"))]
    FullCompliance,
    /// Effects in this mode are always run if item's state is high enough to run it
    #[cfg_attr(feature = "serde", serde(rename = "state"))]
    StateCompliance,
    /// Effects in this mode are always running no matter what
    #[cfg_attr(feature = "serde", serde(rename = "run"))]
    ForceRun,
    /// Effects in this mode are never running no matter what
    #[cfg_attr(feature = "serde", serde(rename = "stop"))]
    ForceStop,
}
