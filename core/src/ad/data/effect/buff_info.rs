use crate::defs::{EAttrId, EBuffId, Rational};

/// Effect-specific buff information.
pub struct AEffectBuffInfo {
    /// Defines where to look for buff type and value.
    pub data_source: AEffectBuffDataSrc,
    /// Defines what items the buff is applied to.
    pub scope: AEffectBuffScope,
}
impl AEffectBuffInfo {
    /// Make a new adapted dogma effect out of passed data.
    pub(crate) fn new(data_source: AEffectBuffDataSrc, scope: AEffectBuffScope) -> Self {
        Self { data_source, scope }
    }
}

/// Defines what items the buff is applied to.
pub enum AEffectBuffScope {
    /// Directly affects all items the effect is applied to.
    Everything,
    /// Affects only ships.
    Ships,
    /// Affects only ships in the same fleet as buff carrier.
    FleetShips,
}

/// Defines where to look for buff type and value.
pub enum AEffectBuffDataSrc {
    /// Standard set of attributes on carrying item.
    DefaultAttrs,
    /// Hardcoded buff ID, but buff value is stored on source item.
    HardcodedId(EBuffId, EAttrId),
    /// Hardcoded buff ID and buff value for the effect.
    HardcodedAll(EBuffId, Rational),
}
