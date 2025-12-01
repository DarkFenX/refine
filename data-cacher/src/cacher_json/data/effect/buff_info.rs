use crate::cacher_json::data::{CAttrId, CAttrVal, CBuffId, CItemListId};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json) struct CEffectBuffInfo {
    default_attrs: Option<CEffectBuffScope>,
    custom: Vec<CEffectBuffCustom>,
}
impl From<&rc::ad::AEffectBuffInfo> for CEffectBuffInfo {
    fn from(a_buff_info: &rc::ad::AEffectBuffInfo) -> Self {
        Self {
            default_attrs: a_buff_info.default_attrs.as_ref().map(Into::into),
            custom: a_buff_info.custom.iter().map(Into::into).collect(),
        }
    }
}
impl From<&CEffectBuffInfo> for rc::ad::AEffectBuffInfo {
    fn from(c_buff_info: &CEffectBuffInfo) -> Self {
        Self {
            default_attrs: c_buff_info.default_attrs.as_ref().map(Into::into),
            custom: c_buff_info.custom.iter().map(Into::into).collect(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json) struct CEffectBuffCustom {
    buff_id: CBuffId,
    source: CEffectBuffCustomSrc,
    scope: CEffectBuffScope,
}
impl From<&rc::ad::AEffectBuffCustom> for CEffectBuffCustom {
    fn from(a_buff_custom: &rc::ad::AEffectBuffCustom) -> Self {
        Self {
            buff_id: a_buff_custom.buff_id,
            source: (&a_buff_custom.source).into(),
            scope: (&a_buff_custom.scope).into(),
        }
    }
}
impl From<&CEffectBuffCustom> for rc::ad::AEffectBuffCustom {
    fn from(c_buff_custom: &CEffectBuffCustom) -> Self {
        Self {
            buff_id: c_buff_custom.buff_id,
            source: (&c_buff_custom.source).into(),
            scope: (&c_buff_custom.scope).into(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cacher_json) enum CEffectBuffCustomSrc {
    Attr(CAttrId),
    Hardcoded(CAttrVal),
}
impl From<&rc::ad::AEffectBuffCustomSrc> for CEffectBuffCustomSrc {
    fn from(a_buff_src: &rc::ad::AEffectBuffCustomSrc) -> Self {
        match a_buff_src {
            rc::ad::AEffectBuffCustomSrc::Attr(attr_id) => Self::Attr(*attr_id),
            rc::ad::AEffectBuffCustomSrc::Hardcoded(buff_val) => Self::Hardcoded(*buff_val),
        }
    }
}
impl From<&CEffectBuffCustomSrc> for rc::ad::AEffectBuffCustomSrc {
    fn from(c_buff_src: &CEffectBuffCustomSrc) -> Self {
        match c_buff_src {
            CEffectBuffCustomSrc::Attr(attr_id) => Self::Attr(*attr_id),
            CEffectBuffCustomSrc::Hardcoded(buff_val) => Self::Hardcoded(*buff_val),
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
