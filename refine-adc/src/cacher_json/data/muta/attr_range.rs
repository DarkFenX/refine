use crate::cacher_json::data::AdaptedConv;

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(super) struct CMutaAttrRange {
    min_mult: f64,
    max_mult: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for CMutaAttrRange {
    type AEntity = rc::ad::AMutaAttrRange;

    fn from_adapted(a_muta_range: &Self::AEntity) -> Self {
        Self {
            min_mult: a_muta_range.min_mult.into_f64(),
            max_mult: a_muta_range.max_mult.into_f64(),
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        Self::AEntity {
            min_mult: rc::ad::AValue::from_f64(self.min_mult),
            max_mult: rc::ad::AValue::from_f64(self.max_mult),
        }
    }
}
