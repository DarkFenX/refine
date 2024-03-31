use crate::handler_json::data::CBuffTgtFilter;

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CBuffAttrMod {
    tgt_filter: CBuffTgtFilter,
    tgt_attr_id: rc::EAttrId,
}
impl From<&rc::ad::ABuffAttrMod> for CBuffAttrMod {
    fn from(a_modifier: &rc::ad::ABuffAttrMod) -> Self {
        CBuffAttrMod {
            tgt_filter: (&a_modifier.tgt_filter).into(),
            tgt_attr_id: a_modifier.tgt_attr_id,
        }
    }
}
impl Into<rc::ad::ABuffAttrMod> for &CBuffAttrMod {
    fn into(self) -> rc::ad::ABuffAttrMod {
        rc::ad::ABuffAttrMod {
            tgt_filter: (&self.tgt_filter).into(),
            tgt_attr_id: self.tgt_attr_id,
        }
    }
}
