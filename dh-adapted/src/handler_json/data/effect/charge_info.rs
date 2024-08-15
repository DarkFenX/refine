#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub struct CEffectChargeInfo {
    pub location: CEffectChargeLocation,
    pub run_effects: bool,
}
impl From<&rc::ad::AEffectChargeInfo> for CEffectChargeInfo {
    fn from(a_charge_info: &rc::ad::AEffectChargeInfo) -> Self {
        Self {
            location: (&a_charge_info.location).into(),
            run_effects: a_charge_info.run_effects,
        }
    }
}
impl Into<rc::ad::AEffectChargeInfo> for &CEffectChargeInfo {
    fn into(self) -> rc::ad::AEffectChargeInfo {
        rc::ad::AEffectChargeInfo {
            location: (&self.location).into(),
            run_effects: self.run_effects,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CEffectChargeLocation {
    Loaded,
    Attr(rc::EAttrId),
}
impl From<&rc::ad::AEffectChargeLocation> for CEffectChargeLocation {
    fn from(a_charge_location: &rc::ad::AEffectChargeLocation) -> Self {
        match a_charge_location {
            rc::ad::AEffectChargeLocation::Loaded => Self::Loaded,
            rc::ad::AEffectChargeLocation::Attr(attr_id) => Self::Attr(*attr_id),
        }
    }
}
impl Into<rc::ad::AEffectChargeLocation> for &CEffectChargeLocation {
    fn into(self) -> rc::ad::AEffectChargeLocation {
        match self {
            CEffectChargeLocation::Loaded => rc::ad::AEffectChargeLocation::Loaded,
            CEffectChargeLocation::Attr(attr_id) => rc::ad::AEffectChargeLocation::Attr(*attr_id),
        }
    }
}
