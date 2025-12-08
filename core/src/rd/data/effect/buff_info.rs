use crate::{
    ad::{
        AAttrId, AAttrVal, ABuffId, AEffectBuffAttrMerge, AEffectBuffDuration, AEffectBuffFull, AEffectBuffInfo,
        AEffectBuffScope, AEffectBuffStrength, AItemListId,
    },
    rd::{RAttrKey, RBuffKey, RItemListKey},
    util::RMap,
};

pub(crate) struct REffectBuffInfo {
    pub(crate) attr_merge: Option<REffectBuffAttrMerge>,
    pub(crate) full: Vec<REffectBuffFull>,
}
impl REffectBuffInfo {
    pub(in crate::rd::data::effect) fn try_from_a_buff(
        a_buff: &AEffectBuffInfo,
        item_list_id_key_map: &RMap<AItemListId, RItemListKey>,
        attr_id_key_map: &RMap<AAttrId, RAttrKey>,
        buff_id_key_map: &RMap<ABuffId, RBuffKey>,
    ) -> Option<Self> {
        let r_buff = Self {
            attr_merge: a_buff.attr_merge.as_ref().and_then(|v| {
                REffectBuffAttrMerge::try_from_a_buff_attr_merge(v, item_list_id_key_map, attr_id_key_map)
            }),
            full: a_buff
                .full
                .iter()
                .filter_map(|v| {
                    REffectBuffFull::try_from_a_buff_full(v, item_list_id_key_map, attr_id_key_map, buff_id_key_map)
                })
                .collect(),
        };
        match r_buff.attr_merge.is_none() && r_buff.full.is_empty() {
            true => None,
            false => Some(r_buff),
        }
    }
}

pub(crate) struct REffectBuffAttrMerge {
    pub(crate) duration: REffectBuffDuration,
    pub(crate) scope: REffectBuffScope,
}
impl REffectBuffAttrMerge {
    fn try_from_a_buff_attr_merge(
        a_buff_attr_merge: &AEffectBuffAttrMerge,
        item_list_id_key_map: &RMap<AItemListId, RItemListKey>,
        attr_id_key_map: &RMap<AAttrId, RAttrKey>,
    ) -> Option<Self> {
        Some(Self {
            duration: REffectBuffDuration::from_a_buff_duration(&a_buff_attr_merge.duration, attr_id_key_map),
            scope: REffectBuffScope::try_from_a_buff_scope(&a_buff_attr_merge.scope, item_list_id_key_map)?,
        })
    }
}

pub(crate) struct REffectBuffFull {
    pub(crate) buff_key: RBuffKey,
    pub(crate) strength: REffectBuffStrength,
    pub(crate) duration: REffectBuffDuration,
    pub(crate) scope: REffectBuffScope,
}
impl REffectBuffFull {
    fn try_from_a_buff_full(
        a_buff_full: &AEffectBuffFull,
        item_list_id_key_map: &RMap<AItemListId, RItemListKey>,
        attr_id_key_map: &RMap<AAttrId, RAttrKey>,
        buff_id_key_map: &RMap<ABuffId, RBuffKey>,
    ) -> Option<Self> {
        Some(Self {
            buff_key: *buff_id_key_map.get(&a_buff_full.buff_id)?,
            strength: REffectBuffStrength::try_from_a_buff_strength(&a_buff_full.strength, attr_id_key_map)?,
            duration: REffectBuffDuration::from_a_buff_duration(&a_buff_full.duration, attr_id_key_map),
            scope: REffectBuffScope::try_from_a_buff_scope(&a_buff_full.scope, item_list_id_key_map)?,
        })
    }
}

pub(crate) enum REffectBuffStrength {
    Attr(RAttrKey),
    Hardcoded(AAttrVal),
}
impl REffectBuffStrength {
    fn try_from_a_buff_strength(
        a_buff_strength: &AEffectBuffStrength,
        attr_id_key_map: &RMap<AAttrId, RAttrKey>,
    ) -> Option<Self> {
        match a_buff_strength {
            AEffectBuffStrength::Attr(attr_id) => Some(Self::Attr(*attr_id_key_map.get(attr_id)?)),
            AEffectBuffStrength::Hardcoded(val) => Some(Self::Hardcoded(*val)),
        }
    }
}

pub(crate) enum REffectBuffDuration {
    None,
    AttrMs(RAttrKey),
}
impl REffectBuffDuration {
    fn from_a_buff_duration(a_buff_duration: &AEffectBuffDuration, attr_id_key_map: &RMap<AAttrId, RAttrKey>) -> Self {
        match a_buff_duration {
            AEffectBuffDuration::None => Self::None,
            // Unlike other buff info parts, do not fail conversion when attribute does not exist
            // in data. The lib does not use duration in any way anyway.
            AEffectBuffDuration::AttrMs(attr_id) => match attr_id_key_map.get(attr_id) {
                Some(attr_key) => Self::AttrMs(*attr_key),
                None => Self::None,
            },
        }
    }
}

pub(crate) enum REffectBuffScope {
    Carrier,
    Projected(RItemListKey),
    Fleet(RItemListKey),
}
impl REffectBuffScope {
    fn try_from_a_buff_scope(
        a_buff_scope: &AEffectBuffScope,
        item_list_id_key_map: &RMap<AItemListId, RItemListKey>,
    ) -> Option<Self> {
        match a_buff_scope {
            AEffectBuffScope::Carrier => Some(Self::Carrier),
            AEffectBuffScope::Projected(item_list_id) => {
                Some(Self::Projected(*item_list_id_key_map.get(item_list_id)?))
            }
            AEffectBuffScope::Fleet(item_list_id) => Some(Self::Fleet(*item_list_id_key_map.get(item_list_id)?)),
        }
    }
}
