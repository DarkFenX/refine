use crate::ad::AValue;

#[cfg_attr(
    feature = "serde-ad",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
pub struct AMutaAttrRange {
    pub mult_min: AValue,
    pub mult_max: AValue,
}
