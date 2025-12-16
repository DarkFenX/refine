use crate::{
    ad::{ABuffId, AEffect, AEffectBuff, AEffectBuffDuration, AEffectBuffScope, AEffectBuffStrength, AEffectId},
    ed::{EAttrId, EBuffId, EEffectId, EItemId, EItemListId},
};

impl AEffect {
    pub(in crate::ad::generator::rels) fn iter_e_item_ids(&self) -> impl Iterator<Item = EItemId> {
        self.id.dc_eve_item().into_iter()
    }
    pub(in crate::ad::generator::rels) fn iter_e_item_list_ids(&self) -> impl Iterator<Item = EItemListId> {
        self.buff.as_ref().into_iter().flat_map(|v| v.iter_e_item_list_ids())
    }
    pub(in crate::ad::generator::rels) fn iter_e_attr_ids(&self) -> impl Iterator<Item = EAttrId> {
        let buff = self.buff.as_ref().into_iter().flat_map(|v| v.iter_e_attr_ids());
        let discharge = self.discharge_attr_id.and_then(|v| v.dc_eve()).into_iter();
        let duration = self.duration_attr_id.and_then(|v| v.dc_eve()).into_iter();
        let range = self.range_attr_id.and_then(|v| v.dc_eve()).into_iter();
        let falloff = self.falloff_attr_id.and_then(|v| v.dc_eve()).into_iter();
        let track = self.track_attr_id.and_then(|v| v.dc_eve()).into_iter();
        let chance = self.chance_attr_id.and_then(|v| v.dc_eve()).into_iter();
        let resist = self.resist_attr_id.and_then(|v| v.dc_eve()).into_iter();
        buff.chain(discharge)
            .chain(duration)
            .chain(range)
            .chain(falloff)
            .chain(track)
            .chain(chance)
            .chain(resist)
    }
    pub(in crate::ad::generator::rels) fn iter_e_effect_ids(&self) -> impl Iterator<Item = EEffectId> {
        let id = self.id.dc_eve_effect().into_iter();
        let stopped = self.stopped_effect_ids.iter().filter_map(|v| v.dc_eve_effect());
        id.chain(stopped)
    }
    pub(in crate::ad::generator::rels) fn iter_e_buff_ids(&self) -> impl Iterator<Item = EBuffId> {
        self.buff.as_ref().into_iter().flat_map(|v| v.iter_e_buff_ids())
    }
}

impl AEffectId {
    fn dc_eve_item(&self) -> Option<EItemId> {
        match self {
            Self::ScSystemWide(e_item_id)
            | Self::ScSystemEmitter(e_item_id)
            | Self::ScProxyEffect(e_item_id)
            | Self::ScProxyTrap(e_item_id)
            | Self::ScShipLink(e_item_id) => Some(*e_item_id),
            Self::Dogma(_) | Self::Custom(_) => None,
        }
    }
    fn dc_eve_effect(&self) -> Option<EEffectId> {
        match self {
            Self::Dogma(e_effect_id) => Some(*e_effect_id),
            Self::ScSystemWide(_)
            | Self::ScSystemEmitter(_)
            | Self::ScProxyEffect(_)
            | Self::ScProxyTrap(_)
            | Self::ScShipLink(_)
            | Self::Custom(_) => None,
        }
    }
}

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
