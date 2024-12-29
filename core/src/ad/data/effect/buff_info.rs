use crate::defs::{AttrVal, EAttrId, EBuffId};

/// Effect-specific buff information.
#[derive(Clone)]
pub struct AEffectBuffInfo {
    /// Defines where to look for buff type and value.
    pub source: AEffectBuffSrc,
    /// Defines what items the buff is applied to.
    pub scope: AEffectBuffScope,
}
impl AEffectBuffInfo {
    pub(crate) fn new(source: AEffectBuffSrc, scope: AEffectBuffScope) -> Self {
        Self { source, scope }
    }
}

/// Defines what items the buff is applied to.
#[derive(Copy, Clone)]
pub enum AEffectBuffScope {
    /// Directly affects all items the effect is applied to, except for structures.
    Everything,
    /// Affects only ships.
    Ships,
    /// Affects only ships in the same fleet as buff carrier.
    FleetShips,
}

/// Defines where to look for buff type and value.
#[derive(Clone)]
pub enum AEffectBuffSrc {
    /// Standard set of attributes on affecting item.
    DefaultAttrs,
    /// Buff ID and values come from elsewhere.
    Customized(Vec<AEffectBuffSrcCustom>),
}

#[derive(Copy, Clone)]
pub enum AEffectBuffSrcCustom {
    /// Hardcoded buff ID, but buff value is stored on affecting item.
    AffectorVal(EBuffId, EAttrId),
    /// Hardcoded buff ID and buff value for the effect.
    HardcodedVal(EBuffId, AttrVal),
}
