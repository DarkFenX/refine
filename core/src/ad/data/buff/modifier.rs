use crate::ad::{AAttrId, ABuffAffecteeFilter};

pub struct ABuffModifier {
    pub affectee_filter: ABuffAffecteeFilter,
    pub affectee_attr_id: AAttrId,
}
