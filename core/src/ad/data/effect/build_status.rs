use crate::defs::Count;

/// Effect modifier build statuses.
///
/// During conversion of EVE data into adapted data, some modifiers might not make it due to various
/// reasons Variants of this enum are stored on an effect, to keep info about conversion status.
pub enum AEffectModBuildStatus {
    /// Modifiers haven't been built yet.
    Unbuilt,
    /// All modifiers failed conversion, with a failure count.
    Error(Count),
    /// Some modifiers failed conversion, with a failure count.
    SuccessPartial(Count),
    /// Conversion was successful.
    Success,
    /// Modifiers on an effect were customized by the library.
    Custom,
}
