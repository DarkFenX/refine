use crate::cacher_json::data::CAttrVal;

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json) struct CMutaAttrRange {
    min_mult: CAttrVal,
    max_mult: CAttrVal,
}
impl From<&rc::ad::AMutaAttrRange> for CMutaAttrRange {
    fn from(a_muta_range: &rc::ad::AMutaAttrRange) -> Self {
        Self {
            min_mult: a_muta_range.min_mult,
            max_mult: a_muta_range.max_mult,
        }
    }
}
impl From<&CMutaAttrRange> for rc::ad::AMutaAttrRange {
    fn from(c_muta_range: &CMutaAttrRange) -> Self {
        Self {
            min_mult: c_muta_range.min_mult,
            max_mult: c_muta_range.max_mult,
        }
    }
}
