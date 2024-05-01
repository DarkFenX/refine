use crate::handler_json::data::{CEffectTgtFilter, CModOp};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CEffectAttrMod {
    src_attr_id: rc::EAttrId,
    op: CModOp,
    tgt_filter: CEffectTgtFilter,
    tgt_attr_id: rc::EAttrId,
}
impl From<&rc::ad::AEffectModifier> for CEffectAttrMod {
    fn from(a_modifier: &rc::ad::AEffectModifier) -> Self {
        CEffectAttrMod {
            src_attr_id: a_modifier.affector_attr_id,
            op: (&a_modifier.op).into(),
            tgt_filter: (&a_modifier.affectee_filter).into(),
            tgt_attr_id: a_modifier.affectee_attr_id,
        }
    }
}
impl Into<rc::ad::AEffectModifier> for &CEffectAttrMod {
    fn into(self) -> rc::ad::AEffectModifier {
        rc::ad::AEffectModifier {
            affector_attr_id: self.src_attr_id,
            op: (&self.op).into(),
            affectee_filter: (&self.tgt_filter).into(),
            affectee_attr_id: self.tgt_attr_id,
        }
    }
}
