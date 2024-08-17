#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CEffectChargeInfo {
    Loaded,
    Attr(rc::EAttrId),
}
impl From<&rc::ad::AEffectChargeInfo> for CEffectChargeInfo {
    fn from(a_charge_info: &rc::ad::AEffectChargeInfo) -> Self {
        match a_charge_info {
            rc::ad::AEffectChargeInfo::Loaded => Self::Loaded,
            rc::ad::AEffectChargeInfo::Attr(attr_id) => Self::Attr(*attr_id),
        }
    }
}
impl Into<rc::ad::AEffectChargeInfo> for &CEffectChargeInfo {
    fn into(self) -> rc::ad::AEffectChargeInfo {
        match self {
            CEffectChargeInfo::Loaded => rc::ad::AEffectChargeInfo::Loaded,
            CEffectChargeInfo::Attr(attr_id) => rc::ad::AEffectChargeInfo::Attr(*attr_id),
        }
    }
}
