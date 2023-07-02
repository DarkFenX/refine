use crate::defs::EBuffId;

/// Defines how a modification will be aggregated.
///
/// When in the non-stack mode, multiple values which share the same aggregation mode and the same
/// aggregation key (the mode argument) are converted into a single value.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ModAggrMode {
    /// All modifications are applied.
    Stack,
    /// Min value will be used, from values with provided key.
    Min(EBuffId),
    /// Max value will be used, from values with provided key.
    Max(EBuffId),
}
