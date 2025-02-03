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
impl From<&CEffectChargeInfo> for rc::ad::AEffectChargeInfo {
    fn from(c_charge_info: &CEffectChargeInfo) -> Self {
        match c_charge_info {
            CEffectChargeInfo::Loaded => Self::Loaded,
            CEffectChargeInfo::Attr(attr_id) => Self::Attr(*attr_id),
        }
    }
}
