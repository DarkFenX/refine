use crate::{
    ad::{ABuffId, AEffectBuff, AEffectBuffDuration, AEffectBuffScope, AEffectBuffStrength},
    ed::{EAttrId, EBuffId, EItemListId},
};

impl AEffectBuff {
    pub(in crate::ad::generator::rels) fn iter_e_item_list_ids(&self) -> impl Iterator<Item = EItemListId> {
        self.iter_a_scopes().filter_map(|v| v.get_e_item_list_id())
    }
    pub(in crate::ad::generator::rels) fn iter_e_attr_ids(&self) -> impl Iterator<Item = EAttrId> {
        let attr_merges = self.attr_merge.and_then(|v| v.duration.get_e_attr_id()).into_iter();
        let full_str = self.full.iter().filter_map(|v| v.strength.get_e_attr_id());
        let full_dur = self.full.iter().filter_map(|v| v.duration.get_e_attr_id());
        attr_merges.chain(full_str).chain(full_dur)
    }
    pub(in crate::ad::generator::rels) fn iter_e_buff_ids(&self) -> impl Iterator<Item = EBuffId> {
        self.full.iter().filter_map(|v| match v.buff_id {
            ABuffId::Eve(buff_id) => Some(buff_id),
            ABuffId::Custom(_) => None,
        })
    }
    pub(in crate::ad::generator) fn iter_a_scopes(&self) -> impl Iterator<Item = AEffectBuffScope> {
        let attr_merges = self.attr_merge.map(|v| v.scope).into_iter();
        let full = self.full.iter().map(|v| v.scope);
        attr_merges.chain(full)
    }
}

impl AEffectBuffStrength {
    fn get_e_attr_id(&self) -> Option<EAttrId> {
        match self {
            Self::Attr(a_attr_id) => a_attr_id.dc_eve(),
            Self::Hardcoded(_) => None,
        }
    }
}

impl AEffectBuffDuration {
    fn get_e_attr_id(&self) -> Option<EAttrId> {
        match self {
            Self::None => None,
            Self::AttrMs(a_attr_id) => a_attr_id.dc_eve(),
        }
    }
}

impl AEffectBuffScope {
    fn get_e_item_list_id(&self) -> Option<EItemListId> {
        match self {
            Self::Carrier => None,
            Self::Projected(item_list_id) => item_list_id.dc_eve(),
            Self::Fleet(item_list_id) => item_list_id.dc_eve(),
        }
    }
}
