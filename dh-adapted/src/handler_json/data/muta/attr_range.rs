#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CMutaAttrRange {
    min_mult: rc::AttrVal,
    max_mult: rc::AttrVal,
}
impl From<&rc::ad::AMutaAttrRange> for CMutaAttrRange {
    fn from(a_muta_range: &rc::ad::AMutaAttrRange) -> Self {
        CMutaAttrRange {
            min_mult: a_muta_range.min_mult,
            max_mult: a_muta_range.max_mult,
        }
    }
}
impl Into<rc::ad::AMutaAttrRange> for &CMutaAttrRange {
    fn into(self) -> rc::ad::AMutaAttrRange {
        rc::ad::AMutaAttrRange {
            min_mult: self.min_mult,
            max_mult: self.max_mult,
        }
    }
}
