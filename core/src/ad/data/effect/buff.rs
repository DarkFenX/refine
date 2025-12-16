use crate::ad::{AAttrId, AAttrVal, ABuffId, AItemListId};

#[derive(Clone)]
pub struct AEffectBuff {
    pub attr_merge: Option<AEffectBuffAttrMerge> = None,
    pub full: Vec<AEffectBuffFull> = Vec::new(),
}

#[derive(Copy, Clone)]
pub struct AEffectBuffAttrMerge {
    pub duration: AEffectBuffDuration,
    pub scope: AEffectBuffScope,
}

#[derive(Copy, Clone)]
pub struct AEffectBuffFull {
    pub buff_id: ABuffId,
    pub strength: AEffectBuffStrength,
    pub duration: AEffectBuffDuration,
    pub scope: AEffectBuffScope,
}

#[derive(Copy, Clone)]
pub enum AEffectBuffStrength {
    Attr(AAttrId),
    Hardcoded(AAttrVal),
}

#[derive(Copy, Clone)]
pub enum AEffectBuffDuration {
    None,
    AttrMs(AAttrId),
}

#[derive(Copy, Clone)]
pub enum AEffectBuffScope {
    Carrier,
    Projected(AItemListId),
    Fleet(AItemListId),
}
