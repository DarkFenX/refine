use crate::handler_json::data::{CModOp, CModTgtFilter};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CEffectAttrMod {
    src_attr_id: rc::EAttrId,
    op: CModOp,
    tgt_filter: CModTgtFilter,
    tgt_attr_id: rc::EAttrId,
}
impl From<&rc::ad::AEffectAttrMod> for CEffectAttrMod {
    fn from(a_modifier: &rc::ad::AEffectAttrMod) -> Self {
        CEffectAttrMod {
            src_attr_id: a_modifier.src_attr_id,
            op: (&a_modifier.op).into(),
            tgt_filter: (&a_modifier.tgt_filter).into(),
            tgt_attr_id: a_modifier.tgt_attr_id,
        }
    }
}
impl Into<rc::ad::AEffectAttrMod> for &CEffectAttrMod {
    fn into(self) -> rc::ad::AEffectAttrMod {
        rc::ad::AEffectAttrMod {
            src_attr_id: self.src_attr_id,
            op: (&self.op).into(),
            tgt_filter: (&self.tgt_filter).into(),
            tgt_attr_id: self.tgt_attr_id,
        }
    }
}
