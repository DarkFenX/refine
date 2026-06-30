use crate::cacher_json::data::AdaptedConv;

#[serde_with::serde_as]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum CEffectModStrength {
    Attr(#[serde_as(as = "serde_with::DisplayFromStr")] rc::ad::AAttrId),
    Hardcoded(f64),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for CEffectModStrength {
    type AEntity = rc::ad::AEffectModStrength;

    fn from_adapted(a_buff_str: &Self::AEntity) -> Self {
        match a_buff_str {
            Self::AEntity::Attr(attr_id) => Self::Attr(*attr_id),
            Self::AEntity::Hardcoded(buff_val) => Self::Hardcoded(buff_val.into_f64()),
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        match self {
            Self::Attr(attr_id) => Self::AEntity::Attr(attr_id),
            Self::Hardcoded(buff_val) => Self::AEntity::Hardcoded(rc::ad::AValue::from_f64(buff_val)),
        }
    }
}
