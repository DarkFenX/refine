use super::affectee_filter::CBuffAffecteeFilter;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(super) struct CBuffModifier {
    affectee_filter: CBuffAffecteeFilter,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    affectee_attr_id: rc::ad::AAttrId,
}
impl CBuffModifier {
    pub(super) fn from_adapted(a_modifier: &rc::ad::ABuffModifier) -> Self {
        Self {
            affectee_filter: CBuffAffecteeFilter::from_adapted(&a_modifier.affectee_filter),
            affectee_attr_id: a_modifier.affectee_attr_id,
        }
    }
    pub(super) fn into_adapted(self) -> rc::ad::ABuffModifier {
        rc::ad::ABuffModifier {
            affectee_filter: self.affectee_filter.into_adapted(),
            affectee_attr_id: self.affectee_attr_id,
        }
    }
}
