#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(super) struct CMutaAttrRange {
    min_mult: f64,
    max_mult: f64,
}
impl CMutaAttrRange {
    pub(super) fn from_adapted(a_muta_range: &rc::ad::AMutaAttrRange) -> Self {
        Self {
            min_mult: a_muta_range.min_mult.into_f64(),
            max_mult: a_muta_range.max_mult.into_f64(),
        }
    }
    pub(super) fn into_adapted(self) -> rc::ad::AMutaAttrRange {
        rc::ad::AMutaAttrRange {
            min_mult: rc::ad::AValue::from_f64(self.min_mult),
            max_mult: rc::ad::AValue::from_f64(self.max_mult),
        }
    }
}
