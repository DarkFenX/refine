use crate::ad::{AAttrId, AAttrVal, ABuffId};

#[derive(Clone)]
pub struct AEffectBuffInfo {
    pub source: AEffectBuffSrc,
    pub scope: AEffectBuffScope,
}

#[derive(Copy, Clone)]
pub enum AEffectBuffScope {
    Everything,
    Ships,
    FleetShips,
}

#[derive(Clone)]
pub enum AEffectBuffSrc {
    DefaultAttrs,
    Customized(Vec<AEffectBuffSrcCustom>),
}

#[derive(Copy, Clone)]
pub enum AEffectBuffSrcCustom {
    AffectorVal(ABuffId, AAttrId),
    HardcodedVal(ABuffId, AAttrVal),
}
