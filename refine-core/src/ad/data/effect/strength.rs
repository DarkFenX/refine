use crate::ad::{AAttrId, AValue};

#[cfg_attr(
    feature = "serde-ad",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[derive(Copy, Clone, PartialEq)]
pub enum AEffectModStrength {
    Attr(AAttrId),
    Hardcoded(AValue),
}
