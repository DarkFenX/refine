use crate::{ad, rd::RBuffKey};

/// Effect-specific buff information.
pub struct REffectBuffInfo {
    /// Defines where to look for buff type and value.
    pub source: REffectBuffSrc,
    /// Defines what items the buff is applied to.
    pub scope: ad::AEffectBuffScope,
}

/// Defines where to look for buff type and value.
pub enum REffectBuffSrc {
    /// Standard set of attributes on affecting item.
    DefaultAttrs,
    /// Buff ID and values come from elsewhere.
    Customized(Vec<REffectBuffSrcCustom>),
}

pub enum REffectBuffSrcCustom {
    /// Hardcoded buff ID, but buff value is stored on affecting item.
    AffectorVal(RBuffKey, ad::AAttrId),
    /// Hardcoded buff ID and buff value for the effect.
    HardcodedVal(RBuffKey, ad::AAttrVal),
}
