use crate::cacher_json::data::{CAttrId, CBuffAffecteeFilter};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json) struct CBuffModifier {
    affectee_filter: CBuffAffecteeFilter,
    affectee_attr_id: CAttrId,
}
impl From<&rc::ad::ABuffModifier> for CBuffModifier {
    fn from(a_modifier: &rc::ad::ABuffModifier) -> Self {
        Self {
            affectee_filter: (&a_modifier.affectee_filter).into(),
            affectee_attr_id: (&a_modifier.affectee_attr_id).into(),
        }
    }
}
impl From<&CBuffModifier> for rc::ad::ABuffModifier {
    fn from(c_modifier: &CBuffModifier) -> Self {
        Self {
            affectee_filter: (&c_modifier.affectee_filter).into(),
            affectee_attr_id: (&c_modifier.affectee_attr_id).into(),
        }
    }
}
