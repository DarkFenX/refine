use crate::handler_json::data::CBuffAffecteeFilter;

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CBuffModifier {
    affectee_filter: CBuffAffecteeFilter,
    affectee_attr_id: rc::EAttrId,
}
impl From<&rc::ad::ABuffModifier> for CBuffModifier {
    fn from(a_modifier: &rc::ad::ABuffModifier) -> Self {
        CBuffModifier {
            affectee_filter: (&a_modifier.affectee_filter).into(),
            affectee_attr_id: a_modifier.affectee_attr_id,
        }
    }
}
impl Into<rc::ad::ABuffModifier> for &CBuffModifier {
    fn into(self) -> rc::ad::ABuffModifier {
        rc::ad::ABuffModifier {
            affectee_filter: (&self.affectee_filter).into(),
            affectee_attr_id: self.affectee_attr_id,
        }
    }
}
