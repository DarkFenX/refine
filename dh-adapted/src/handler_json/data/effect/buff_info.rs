#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CEffectBuffInfo {
    data_source: CEffectBuffDataSrc,
    scope: CEffectBuffScope,
}
impl From<&rc::ad::AEffectBuffInfo> for CEffectBuffInfo {
    fn from(a_buff_info: &rc::ad::AEffectBuffInfo) -> Self {
        Self {
            data_source: (&a_buff_info.data_source).into(),
            scope: (&a_buff_info.scope).into(),
        }
    }
}
impl Into<rc::ad::AEffectBuffInfo> for &CEffectBuffInfo {
    fn into(self) -> rc::ad::AEffectBuffInfo {
        rc::ad::AEffectBuffInfo {
            data_source: (&self.data_source).into(),
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
pub(in crate::handler_json) enum CEffectBuffDataSrc {
    DefaultAttrs,
    Hardcoded(rc::EBuffId, rc::AttrVal),
}
impl From<&rc::ad::AEffectBuffDataSrc> for CEffectBuffDataSrc {
    fn from(buff_data_src: &rc::ad::AEffectBuffDataSrc) -> Self {
        match buff_data_src {
            rc::ad::AEffectBuffDataSrc::DefaultAttrs => Self::DefaultAttrs,
            rc::ad::AEffectBuffDataSrc::Hardcoded(buff_id, buff_val) => Self::Hardcoded(*buff_id, *buff_val),
        }
    }
}
impl Into<rc::ad::AEffectBuffDataSrc> for &CEffectBuffDataSrc {
    fn into(self) -> rc::ad::AEffectBuffDataSrc {
        match self {
            CEffectBuffDataSrc::DefaultAttrs => rc::ad::AEffectBuffDataSrc::DefaultAttrs,
            CEffectBuffDataSrc::Hardcoded(buff_id, buff_val) => {
                rc::ad::AEffectBuffDataSrc::Hardcoded(*buff_id, *buff_val)
            }
        }
    }
}
