use crate::ad::{AAttrId, AAttrVal, ABuffId, AItemListId};

/// Effect-specific buff info.
#[derive(Clone)]
pub struct AEffectBuffInfo {
    pub attr_merge: Option<AEffectBuffAttrMerge> = None,
    pub full: Vec<AEffectBuffFull> = Vec::new(),
}
impl AEffectBuffInfo {
    pub(crate) fn iter_a_item_list_ids(&self) -> impl Iterator<Item = AItemListId> {
        let attr_merges = self.attr_merge.and_then(|v| v.scope.get_a_item_list_id()).into_iter();
        let full = self.full.iter().filter_map(|v| v.scope.get_a_item_list_id());
        attr_merges.chain(full)
    }
    pub(crate) fn iter_a_attr_ids(&self) -> impl Iterator<Item = AAttrId> {
        let attr_merges = self.attr_merge.and_then(|v| v.duration.get_a_attr_id()).into_iter();
        let full_str = self.full.iter().filter_map(|v| v.strength.get_a_attr_id());
        let full_dur = self.full.iter().filter_map(|v| v.duration.get_a_attr_id());
        attr_merges.chain(full_str).chain(full_dur)
    }
    pub(crate) fn iter_a_buff_ids(&self) -> impl Iterator<Item = ABuffId> {
        self.full.iter().map(|v| v.buff_id)
    }
}

/// Specifies how effect uses warfareBuff* series of attributes, which define buff ID and buff
/// strength.
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
impl AEffectBuffStrength {
    fn get_a_attr_id(&self) -> Option<AAttrId> {
        match self {
            Self::Attr(attr_id) => Some(*attr_id),
            Self::Hardcoded(_) => None,
        }
    }
}

#[derive(Copy, Clone)]
pub enum AEffectBuffDuration {
    /// Buff is active as long as item which applies it is active.
    None,
    /// Attribute with this ID defines duration in milliseconds.
    AttrMs(AAttrId),
}
impl AEffectBuffDuration {
    fn get_a_attr_id(&self) -> Option<AAttrId> {
        match self {
            Self::None => None,
            Self::AttrMs(attr_id) => Some(*attr_id),
        }
    }
}

#[derive(Copy, Clone)]
pub enum AEffectBuffScope {
    Carrier,
    Projected(AItemListId),
    Fleet(AItemListId),
}
impl AEffectBuffScope {
    pub(crate) fn get_a_item_list_id(&self) -> Option<AItemListId> {
        match self {
            Self::Carrier => None,
            Self::Projected(item_list_id) => Some(*item_list_id),
            Self::Fleet(item_list_id) => Some(*item_list_id),
        }
    }
}
