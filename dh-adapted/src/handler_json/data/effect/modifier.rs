use crate::handler_json::data::{CEffectAffecteeFilter, CModOp};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CEffectModifier {
    affector_attr_id: rc::EAttrId,
    op: CModOp,
    affectee_filter: CEffectAffecteeFilter,
    affectee_attr_id: rc::EAttrId,
}
impl From<&rc::ad::AEffectModifier> for CEffectModifier {
    fn from(a_modifier: &rc::ad::AEffectModifier) -> Self {
        CEffectModifier {
            affector_attr_id: a_modifier.affector_attr_id,
            op: (&a_modifier.op).into(),
            affectee_filter: (&a_modifier.affectee_filter).into(),
            affectee_attr_id: a_modifier.affectee_attr_id,
        }
    }
}
impl Into<rc::ad::AEffectModifier> for &CEffectModifier {
    fn into(self) -> rc::ad::AEffectModifier {
        rc::ad::AEffectModifier {
            affector_attr_id: self.affector_attr_id,
            op: (&self.op).into(),
            affectee_filter: (&self.affectee_filter).into(),
            affectee_attr_id: self.affectee_attr_id,
        }
    }
}
