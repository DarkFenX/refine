use super::{super::shared::COp, affectee_filter::CEffectAffecteeFilter, strength::CEffectModStrength};

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(super) struct CEffectModifier {
    strength: CEffectModStrength,
    op: COp,
    affectee_filter: CEffectAffecteeFilter,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    affectee_attr_id: rc::ad::AAttrId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CEffectModifier {
    pub(super) fn from_adapted(a_modifier: &rc::ad::AEffectModifier) -> Self {
        Self {
            strength: CEffectModStrength::from_adapted(&a_modifier.strength),
            op: COp::from_adapted(&a_modifier.op),
            affectee_filter: CEffectAffecteeFilter::from_adapted(&a_modifier.affectee_filter),
            affectee_attr_id: a_modifier.affectee_attr_id,
        }
    }
    pub(super) fn into_adapted(self) -> rc::ad::AEffectModifier {
        rc::ad::AEffectModifier {
            strength: self.strength.into_adapted(),
            op: self.op.into_adapted(),
            affectee_filter: self.affectee_filter.into_adapted(),
            affectee_attr_id: self.affectee_attr_id,
        }
    }
}
