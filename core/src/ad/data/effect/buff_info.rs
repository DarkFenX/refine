use crate::ad::{AAttrId, AAttrVal, ABuffId, AItemListId};

#[derive(Clone)]
pub struct AEffectBuffInfo {
    pub source: AEffectBuffSrc,
    pub scope: AEffectBuffScope,
}

#[derive(Copy, Clone)]
pub struct AEffectBuffScope {
    item_list_id: AItemListId,
    fleet_only: bool = false,
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
