use std::iter::chain;

use crate::ad::{AAttrId, AAttrVal, ABuffId, AItemListId};

/// Effect-specific buff info.
#[derive(Clone)]
pub struct AEffectBuffInfo {
    /// Specifies how effect uses warfareBuff* series of attributes, which define buff ID and buff
    /// strength.
    pub default_attrs: Option<AEffectBuffScope> = None,
    pub custom: Vec<AEffectBuffCustom> = Vec::new(),
}
impl AEffectBuffInfo {
    pub(crate) fn iter_a_item_list_ids(&self) -> impl Iterator<Item = AItemListId> {
        chain(
            self.default_attrs.and_then(|v| v.get_a_item_list_id()),
            self.custom.iter().filter_map(|v| v.scope.get_a_item_list_id()),
        )
    }
    pub(crate) fn iter_a_attr_ids(&self) -> impl Iterator<Item = AAttrId> {
        self.custom.iter().filter_map(|v| v.source.get_a_attr_id())
    }
    pub(crate) fn iter_a_buff_ids(&self) -> impl Iterator<Item = ABuffId> {
        self.custom.iter().map(|v| v.buff_id)
    }
}

#[derive(Copy, Clone)]
pub struct AEffectBuffCustom {
    pub buff_id: ABuffId,
    pub source: AEffectBuffCustomSrc,
    pub scope: AEffectBuffScope,
}

#[derive(Copy, Clone)]
pub enum AEffectBuffCustomSrc {
    Attr(AAttrId),
    Hardcoded(AAttrVal),
}
impl AEffectBuffCustomSrc {
    fn get_a_attr_id(&self) -> Option<AAttrId> {
        match self {
            Self::Attr(attr_id) => Some(*attr_id),
            Self::Hardcoded(_) => None,
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
