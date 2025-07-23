use crate::cacher_json::data::{CAttrId, CAttrVal, CBuffId};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json) struct CEffectBuffInfo {
    source: CEffectBuffSrc,
    scope: CEffectBuffScope,
}
impl From<&rc::ad::AEffectBuffInfo> for CEffectBuffInfo {
    fn from(a_buff_info: &rc::ad::AEffectBuffInfo) -> Self {
        Self {
            source: (&a_buff_info.source).into(),
            scope: (&a_buff_info.scope).into(),
        }
    }
}
impl From<&CEffectBuffInfo> for rc::ad::AEffectBuffInfo {
    fn from(c_buff_info: &CEffectBuffInfo) -> Self {
        Self {
            source: (&c_buff_info.source).into(),
            scope: (&c_buff_info.scope).into(),
        }
    }
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::cacher_json) enum CEffectBuffScope {
    Everything,
    Ships,
    FleetShips,
}
impl From<&rc::ad::AEffectBuffScope> for CEffectBuffScope {
    fn from(a_buff_scope: &rc::ad::AEffectBuffScope) -> Self {
        match a_buff_scope {
            rc::ad::AEffectBuffScope::Everything => Self::Everything,
            rc::ad::AEffectBuffScope::Ships => Self::Ships,
            rc::ad::AEffectBuffScope::FleetShips => Self::FleetShips,
        }
    }
}
impl From<&CEffectBuffScope> for rc::ad::AEffectBuffScope {
    fn from(c_buff_scope: &CEffectBuffScope) -> Self {
        match c_buff_scope {
            CEffectBuffScope::Everything => Self::Everything,
            CEffectBuffScope::Ships => Self::Ships,
            CEffectBuffScope::FleetShips => Self::FleetShips,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cacher_json) enum CEffectBuffSrc {
    DefaultAttrs,
    Customized(Vec<CEffectBuffSrcCustom>),
}
impl From<&rc::ad::AEffectBuffSrc> for CEffectBuffSrc {
    fn from(a_buff_src: &rc::ad::AEffectBuffSrc) -> Self {
        match a_buff_src {
            rc::ad::AEffectBuffSrc::DefaultAttrs => Self::DefaultAttrs,
            rc::ad::AEffectBuffSrc::Customized(buff_custom_srcs) => {
                Self::Customized(buff_custom_srcs.iter().map(|v| v.into()).collect())
            }
        }
    }
}
impl From<&CEffectBuffSrc> for rc::ad::AEffectBuffSrc {
    fn from(c_buff_src: &CEffectBuffSrc) -> Self {
        match c_buff_src {
            CEffectBuffSrc::DefaultAttrs => Self::DefaultAttrs,
            CEffectBuffSrc::Customized(buff_custom_srcs) => {
                Self::Customized(buff_custom_srcs.iter().map(|v| v.into()).collect())
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(in crate::cacher_json) enum CEffectBuffSrcCustom {
    AffectorVal(CBuffId, CAttrId),
    HardcodedVal(CBuffId, CAttrVal),
}
impl From<&rc::ad::AEffectBuffSrcCustom> for CEffectBuffSrcCustom {
    fn from(a_buff_data_src_custom: &rc::ad::AEffectBuffSrcCustom) -> Self {
        match a_buff_data_src_custom {
            rc::ad::AEffectBuffSrcCustom::AffectorVal(buff_id, attr_id) => Self::AffectorVal(*buff_id, *attr_id),
            rc::ad::AEffectBuffSrcCustom::HardcodedVal(buff_id, buff_val) => Self::HardcodedVal(*buff_id, *buff_val),
        }
    }
}
impl From<&CEffectBuffSrcCustom> for rc::ad::AEffectBuffSrcCustom {
    fn from(c_buff_data_src_custom: &CEffectBuffSrcCustom) -> Self {
        match c_buff_data_src_custom {
            CEffectBuffSrcCustom::AffectorVal(buff_id, attr_id) => Self::AffectorVal(*buff_id, *attr_id),
            CEffectBuffSrcCustom::HardcodedVal(buff_id, buff_val) => Self::HardcodedVal(*buff_id, *buff_val),
        }
    }
}
