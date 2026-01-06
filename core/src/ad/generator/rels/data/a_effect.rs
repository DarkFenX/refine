use crate::{
    ad::{AEffect, AEffectBuff, AEffectBuffDuration, AEffectBuffScope, AEffectBuffStrength, AEffectId},
    ed::{EAttrId, EBuffId, EEffectId, EItemId, EItemListId},
};

impl AEffect {
    pub(in crate::ad::generator::rels) fn iter_item_eids(&self) -> impl Iterator<Item = EItemId> {
        self.id.dc_eve_item().into_iter()
    }
    pub(in crate::ad::generator::rels) fn iter_item_list_eids(&self) -> impl Iterator<Item = EItemListId> {
        self.buff.as_ref().into_iter().flat_map(|v| v.iter_item_list_eids())
    }
    pub(in crate::ad::generator::rels) fn iter_attr_eids(&self) -> impl Iterator<Item = EAttrId> {
        let buff = self.buff.as_ref().into_iter().flat_map(|v| v.iter_attr_eids());
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
    pub(in crate::ad::generator::rels) fn iter_effect_eids(&self) -> impl Iterator<Item = EEffectId> {
        let id = self.id.dc_eve_effect().into_iter();
        let stopped = self.stopped_effect_ids.iter().filter_map(|v| v.dc_eve_effect());
        id.chain(stopped)
    }
    pub(in crate::ad::generator::rels) fn iter_buff_eids(&self) -> impl Iterator<Item = EBuffId> {
        self.buff.as_ref().into_iter().flat_map(|v| v.iter_buff_eids())
    }
}

impl AEffectId {
    fn dc_eve_item(&self) -> Option<EItemId> {
        match self {
            Self::ScSystemWide(item_aid)
            | Self::ScSystemEmitter(item_aid)
            | Self::ScProxyEffect(item_aid)
            | Self::ScProxyTrap(item_aid)
            | Self::ScShipLink(item_aid) => Some(EItemId::from_i32(item_aid.into_i32())),
            Self::Dogma(_) | Self::Custom(_) => None,
        }
    }
    fn dc_eve_effect(&self) -> Option<EEffectId> {
        match self {
            Self::Dogma(dogma_effect_aid) => Some(EEffectId::from_i32(dogma_effect_aid.into_i32())),
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
    pub(in crate::ad::generator::rels) fn iter_item_list_eids(&self) -> impl Iterator<Item = EItemListId> {
        self.iter_a_scopes().filter_map(|v| v.get_item_list_eid())
    }
    pub(in crate::ad::generator::rels) fn iter_attr_eids(&self) -> impl Iterator<Item = EAttrId> {
        let attr_merges = self.attr_merge.and_then(|v| v.duration.get_attr_eid()).into_iter();
        let full_str = self.full.iter().filter_map(|v| v.strength.get_attr_eid());
        let full_dur = self.full.iter().filter_map(|v| v.duration.get_attr_eid());
        attr_merges.chain(full_str).chain(full_dur)
    }
    pub(in crate::ad::generator::rels) fn iter_buff_eids(&self) -> impl Iterator<Item = EBuffId> {
        self.full.iter().filter_map(|v| v.buff_id.dc_eve())
    }
    pub(in crate::ad::generator) fn iter_a_scopes(&self) -> impl Iterator<Item = AEffectBuffScope> {
        let attr_merges = self.attr_merge.map(|v| v.scope).into_iter();
        let full = self.full.iter().map(|v| v.scope);
        attr_merges.chain(full)
    }
}

impl AEffectBuffStrength {
    fn get_attr_eid(&self) -> Option<EAttrId> {
        match self {
            Self::Attr(attr_aid) => attr_aid.dc_eve(),
            Self::Hardcoded(_) => None,
        }
    }
}

impl AEffectBuffDuration {
    fn get_attr_eid(&self) -> Option<EAttrId> {
        match self {
            Self::None => None,
            Self::AttrMs(attr_aid) => attr_aid.dc_eve(),
        }
    }
}

impl AEffectBuffScope {
    fn get_item_list_eid(&self) -> Option<EItemListId> {
        match self {
            Self::Carrier => None,
            Self::Projected(item_list_aid) => item_list_aid.dc_eve(),
            Self::Fleet(item_list_aid) => item_list_aid.dc_eve(),
        }
    }
}
