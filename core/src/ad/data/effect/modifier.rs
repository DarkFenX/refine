use crate::ad::{AAttrId, AEffectAffecteeFilter, AOp};

pub struct AEffectModifier {
    pub affector_attr_id: AAttrId,
    pub op: AOp,
    pub affectee_filter: AEffectAffecteeFilter,
    pub affectee_attr_id: AAttrId,
}
