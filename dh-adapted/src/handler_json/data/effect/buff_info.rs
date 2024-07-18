#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CEffectBuffInfo {
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
impl Into<rc::ad::AEffectBuffInfo> for &CEffectBuffInfo {
    fn into(self) -> rc::ad::AEffectBuffInfo {
        rc::ad::AEffectBuffInfo {
            source: (&self.source).into(),
            scope: (&self.scope).into(),
        }
    }
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CEffectBuffScope {
    Everything,
    Ships,
    FleetShips,
}
impl From<&rc::ad::AEffectBuffScope> for CEffectBuffScope {
    fn from(buff_scope: &rc::ad::AEffectBuffScope) -> Self {
        match buff_scope {
            rc::ad::AEffectBuffScope::Everything => Self::Everything,
            rc::ad::AEffectBuffScope::Ships => Self::Ships,
            rc::ad::AEffectBuffScope::FleetShips => Self::FleetShips,
        }
    }
}
impl Into<rc::ad::AEffectBuffScope> for &CEffectBuffScope {
    fn into(self) -> rc::ad::AEffectBuffScope {
        match self {
            CEffectBuffScope::Everything => rc::ad::AEffectBuffScope::Everything,
            CEffectBuffScope::Ships => rc::ad::AEffectBuffScope::Ships,
            CEffectBuffScope::FleetShips => rc::ad::AEffectBuffScope::FleetShips,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CEffectBuffSrc {
    DefaultAttrs,
    Customized(Vec<CEffectBuffSrcCustom>),
}
impl From<&rc::ad::AEffectBuffSrc> for CEffectBuffSrc {
    fn from(buff_src: &rc::ad::AEffectBuffSrc) -> Self {
        match buff_src {
            rc::ad::AEffectBuffSrc::DefaultAttrs => Self::DefaultAttrs,
            rc::ad::AEffectBuffSrc::Customized(buff_custom_srcs) => {
                Self::Customized(buff_custom_srcs.iter().map(|v| v.into()).collect())
            }
        }
    }
}
impl Into<rc::ad::AEffectBuffSrc> for &CEffectBuffSrc {
    fn into(self) -> rc::ad::AEffectBuffSrc {
        match self {
            CEffectBuffSrc::DefaultAttrs => rc::ad::AEffectBuffSrc::DefaultAttrs,
            CEffectBuffSrc::Customized(buff_custom_srcs) => {
                rc::ad::AEffectBuffSrc::Customized(buff_custom_srcs.iter().map(|v| v.into()).collect())
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(in crate::handler_json) enum CEffectBuffSrcCustom {
    AffectorVal(rc::EBuffId, rc::EAttrId),
    HardcodedVal(rc::EBuffId, rc::Rational),
}
impl From<&rc::ad::AEffectBuffSrcCustom> for CEffectBuffSrcCustom {
    fn from(buff_data_src_custom: &rc::ad::AEffectBuffSrcCustom) -> Self {
        match buff_data_src_custom {
            rc::ad::AEffectBuffSrcCustom::AffectorVal(buff_id, attr_id) => Self::AffectorVal(*buff_id, *attr_id),
            rc::ad::AEffectBuffSrcCustom::HardcodedVal(buff_id, buff_val) => Self::HardcodedVal(*buff_id, *buff_val),
        }
    }
}
impl Into<rc::ad::AEffectBuffSrcCustom> for &CEffectBuffSrcCustom {
    fn into(self) -> rc::ad::AEffectBuffSrcCustom {
        match self {
            CEffectBuffSrcCustom::AffectorVal(buff_id, attr_id) => {
                rc::ad::AEffectBuffSrcCustom::AffectorVal(*buff_id, *attr_id)
            }
            CEffectBuffSrcCustom::HardcodedVal(buff_id, buff_val) => {
                rc::ad::AEffectBuffSrcCustom::HardcodedVal(*buff_id, *buff_val)
            }
        }
    }
}
