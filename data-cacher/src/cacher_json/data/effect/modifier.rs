use super::{affectee_filter::CEffectAffecteeFilter, strength::CEffectModStrength};
use crate::cacher_json::data::{AdaptedConv, COp};

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
impl AdaptedConv for CEffectModifier {
    type AEntity = rc::ad::AEffectModifier;

    fn from_adapted(a_modifier: &Self::AEntity) -> Self {
        Self {
            strength: CEffectModStrength::from_adapted(&a_modifier.strength),
            op: COp::from_adapted(&a_modifier.op),
            affectee_filter: CEffectAffecteeFilter::from_adapted(&a_modifier.affectee_filter),
            affectee_attr_id: a_modifier.affectee_attr_id,
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        Self::AEntity {
            strength: self.strength.into_adapted(),
            op: self.op.into_adapted(),
            affectee_filter: self.affectee_filter.into_adapted(),
            affectee_attr_id: self.affectee_attr_id,
        }
    }
}
