use super::{super::shared::COp, affectee_filter::CEffectAffecteeFilter};

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(super) struct CEffectModifier {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    affector_attr_id: rc::ad::AAttrId,
    op: COp,
    affectee_filter: CEffectAffecteeFilter,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    affectee_attr_id: rc::ad::AAttrId,
}
impl CEffectModifier {
    pub(super) fn from_adapted(a_modifier: &rc::ad::AEffectModifier) -> Self {
        Self {
            affector_attr_id: a_modifier.affector_attr_id,
            op: COp::from_adapted(&a_modifier.op),
            affectee_filter: CEffectAffecteeFilter::from_adapted(&a_modifier.affectee_filter),
            affectee_attr_id: a_modifier.affectee_attr_id,
        }
    }
    pub(super) fn into_adapted(self) -> rc::ad::AEffectModifier {
        rc::ad::AEffectModifier {
            affector_attr_id: self.affector_attr_id,
            op: self.op.into_adapted(),
            affectee_filter: self.affectee_filter.into_adapted(),
            affectee_attr_id: self.affectee_attr_id,
        }
    }
}
