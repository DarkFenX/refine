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
impl CEffectModStrength {
    pub(super) fn from_adapted(a_buff_str: &rc::ad::AEffectModStrength) -> Self {
        match a_buff_str {
            rc::ad::AEffectModStrength::Attr(attr_id) => Self::Attr(*attr_id),
            rc::ad::AEffectModStrength::Hardcoded(buff_val) => Self::Hardcoded(buff_val.into_f64()),
        }
    }
    pub(super) fn into_adapted(self) -> rc::ad::AEffectModStrength {
        match self {
            Self::Attr(attr_id) => rc::ad::AEffectModStrength::Attr(attr_id),
            Self::Hardcoded(buff_val) => rc::ad::AEffectModStrength::Hardcoded(rc::ad::AValue::from_f64(buff_val)),
        }
    }
}
