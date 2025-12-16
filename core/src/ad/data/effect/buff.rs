use crate::{
    ad::{AAttrId, AAttrVal, ABuffId, AItemListId},
    ed::{EAttrId, EBuffId, EItemListId},
};

#[derive(Clone)]
pub struct AEffectBuff {
    pub attr_merge: Option<AEffectBuffAttrMerge> = None,
    pub full: Vec<AEffectBuffFull> = Vec::new(),
}
impl AEffectBuff {
    pub(crate) fn iter_e_item_list_ids(&self) -> impl Iterator<Item = EItemListId> {
        self.iter_a_scopes().filter_map(|v| v.get_e_item_list_id())
    }
    pub(crate) fn iter_e_attr_ids(&self) -> impl Iterator<Item = EAttrId> {
        let attr_merges = self.attr_merge.and_then(|v| v.duration.get_e_attr_id()).into_iter();
        let full_str = self.full.iter().filter_map(|v| v.strength.get_e_attr_id());
        let full_dur = self.full.iter().filter_map(|v| v.duration.get_e_attr_id());
        attr_merges.chain(full_str).chain(full_dur)
    }
    pub(crate) fn iter_e_buff_ids(&self) -> impl Iterator<Item = EBuffId> {
        self.full.iter().filter_map(|v| match v.buff_id {
            ABuffId::Eve(buff_id) => Some(buff_id),
            ABuffId::Custom(_) => None,
        })
    }
    pub(crate) fn iter_a_scopes(&self) -> impl Iterator<Item = AEffectBuffScope> {
        let attr_merges = self.attr_merge.map(|v| v.scope).into_iter();
        let full = self.full.iter().map(|v| v.scope);
        attr_merges.chain(full)
    }
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
impl AEffectBuffStrength {
    fn get_e_attr_id(&self) -> Option<EAttrId> {
        match self {
            Self::Attr(a_attr_id) => a_attr_id.get_e_attr_id(),
            Self::Hardcoded(_) => None,
        }
    }
}

#[derive(Copy, Clone)]
pub enum AEffectBuffDuration {
    None,
    AttrMs(AAttrId),
}
impl AEffectBuffDuration {
    fn get_e_attr_id(&self) -> Option<EAttrId> {
        match self {
            Self::None => None,
            Self::AttrMs(a_attr_id) => a_attr_id.get_e_attr_id(),
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
    pub(crate) fn get_e_item_list_id(&self) -> Option<EItemListId> {
        match self {
            Self::Carrier => None,
            Self::Projected(item_list_id) => item_list_id.dc_eve(),
            Self::Fleet(item_list_id) => item_list_id.dc_eve(),
        }
    }
}
