use crate::cacher_json::data::{CAttrId, CAttrVal, CBuffId, CItemListId};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json) struct CEffectBuffInfo {
    attr_merge: Option<CEffectBuffAttrMerge>,
    full: Vec<CEffectBuffFull>,
}
impl From<&rc::ad::AEffectBuff> for CEffectBuffInfo {
    fn from(a_effect_buff: &rc::ad::AEffectBuff) -> Self {
        Self {
            attr_merge: a_effect_buff.attr_merge.as_ref().map(Into::into),
            full: a_effect_buff.full.iter().map(Into::into).collect(),
        }
    }
}
impl From<&CEffectBuffInfo> for rc::ad::AEffectBuff {
    fn from(c_effect_buff: &CEffectBuffInfo) -> Self {
        Self {
            attr_merge: c_effect_buff.attr_merge.as_ref().map(Into::into),
            full: c_effect_buff.full.iter().map(Into::into).collect(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
struct CEffectBuffAttrMerge {
    duration: CEffectBuffDuration,
    scope: CEffectBuffScope,
}
impl From<&rc::ad::AEffectBuffAttrMerge> for CEffectBuffAttrMerge {
    fn from(a_buff_attr_merge: &rc::ad::AEffectBuffAttrMerge) -> Self {
        Self {
            duration: (&a_buff_attr_merge.duration).into(),
            scope: (&a_buff_attr_merge.scope).into(),
        }
    }
}
impl From<&CEffectBuffAttrMerge> for rc::ad::AEffectBuffAttrMerge {
    fn from(c_buff_attr_merge: &CEffectBuffAttrMerge) -> Self {
        Self {
            duration: (&c_buff_attr_merge.duration).into(),
            scope: (&c_buff_attr_merge.scope).into(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json) struct CEffectBuffFull {
    buff_id: CBuffId,
    strength: CEffectBuffStrength,
    duration: CEffectBuffDuration,
    scope: CEffectBuffScope,
}
impl From<&rc::ad::AEffectBuffFull> for CEffectBuffFull {
    fn from(a_buff_full: &rc::ad::AEffectBuffFull) -> Self {
        Self {
            buff_id: (&a_buff_full.buff_id).into(),
            strength: (&a_buff_full.strength).into(),
            duration: (&a_buff_full.duration).into(),
            scope: (&a_buff_full.scope).into(),
        }
    }
}
impl From<&CEffectBuffFull> for rc::ad::AEffectBuffFull {
    fn from(c_buff_full: &CEffectBuffFull) -> Self {
        Self {
            buff_id: (&c_buff_full.buff_id).into(),
            strength: (&c_buff_full.strength).into(),
            duration: (&c_buff_full.duration).into(),
            scope: (&c_buff_full.scope).into(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cacher_json) enum CEffectBuffStrength {
    Attr(CAttrId),
    Hardcoded(CAttrVal),
}
impl From<&rc::ad::AEffectBuffStrength> for CEffectBuffStrength {
    fn from(a_buff_str: &rc::ad::AEffectBuffStrength) -> Self {
        match a_buff_str {
            rc::ad::AEffectBuffStrength::Attr(attr_id) => Self::Attr(attr_id.into()),
            rc::ad::AEffectBuffStrength::Hardcoded(buff_val) => Self::Hardcoded(*buff_val),
        }
    }
}
impl From<&CEffectBuffStrength> for rc::ad::AEffectBuffStrength {
    fn from(c_buff_str: &CEffectBuffStrength) -> Self {
        match c_buff_str {
            CEffectBuffStrength::Attr(attr_id) => Self::Attr(attr_id.into()),
            CEffectBuffStrength::Hardcoded(buff_val) => Self::Hardcoded(*buff_val),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum CEffectBuffDuration {
    None,
    AttrMs(CAttrId),
}
impl From<&rc::ad::AEffectBuffDuration> for CEffectBuffDuration {
    fn from(a_buff_dur: &rc::ad::AEffectBuffDuration) -> Self {
        match a_buff_dur {
            rc::ad::AEffectBuffDuration::None => Self::None,
            rc::ad::AEffectBuffDuration::AttrMs(attr_id) => Self::AttrMs(attr_id.into()),
        }
    }
}
impl From<&CEffectBuffDuration> for rc::ad::AEffectBuffDuration {
    fn from(c_buff_dur: &CEffectBuffDuration) -> Self {
        match c_buff_dur {
            CEffectBuffDuration::None => Self::None,
            CEffectBuffDuration::AttrMs(attr_id) => Self::AttrMs(attr_id.into()),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cacher_json) enum CEffectBuffScope {
    Carrier,
    Projected(CItemListId),
    Fleet(CItemListId),
}
impl From<&rc::ad::AEffectBuffScope> for CEffectBuffScope {
    fn from(a_buff_scope: &rc::ad::AEffectBuffScope) -> Self {
        match a_buff_scope {
            rc::ad::AEffectBuffScope::Carrier => Self::Carrier,
            rc::ad::AEffectBuffScope::Projected(a_item_list_id) => Self::Projected(a_item_list_id.into()),
            rc::ad::AEffectBuffScope::Fleet(a_item_list_id) => Self::Fleet(a_item_list_id.into()),
        }
    }
}
impl From<&CEffectBuffScope> for rc::ad::AEffectBuffScope {
    fn from(c_buff_scope: &CEffectBuffScope) -> Self {
        match c_buff_scope {
            CEffectBuffScope::Carrier => Self::Carrier,
            CEffectBuffScope::Projected(c_item_list_id) => Self::Projected(c_item_list_id.into()),
            CEffectBuffScope::Fleet(c_item_list_id) => Self::Fleet(c_item_list_id.into()),
        }
    }
}
