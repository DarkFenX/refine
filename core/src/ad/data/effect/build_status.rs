use crate::defs::Amount;

/// Effect modifier build statuses.
///
/// During cache generation, the library converts modifiers of an effect into internal format.
/// Some of those modifiers might not make it through conversion process due to various reasons.
/// Variants of this enum are stored on an effect, to keep info about conversion status.
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
