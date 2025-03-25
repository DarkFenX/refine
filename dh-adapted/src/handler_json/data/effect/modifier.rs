use crate::handler_json::data::{CAttrId, CEffectAffecteeFilter, COp};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CEffectModifier {
    affector_attr_id: CAttrId,
    op: COp,
    affectee_filter: CEffectAffecteeFilter,
    affectee_attr_id: CAttrId,
}
impl From<&rc::ad::AEffectModifier> for CEffectModifier {
    fn from(a_modifier: &rc::ad::AEffectModifier) -> Self {
        Self {
            affector_attr_id: a_modifier.affector_attr_id,
            op: (&a_modifier.op).into(),
            affectee_filter: (&a_modifier.affectee_filter).into(),
            affectee_attr_id: a_modifier.affectee_attr_id,
        }
    }
}
impl From<&CEffectModifier> for rc::ad::AEffectModifier {
    fn from(c_modifier: &CEffectModifier) -> Self {
        Self {
            affector_attr_id: c_modifier.affector_attr_id,
            op: (&c_modifier.op).into(),
            affectee_filter: (&c_modifier.affectee_filter).into(),
            affectee_attr_id: c_modifier.affectee_attr_id,
        }
    }
}
