use crate::{ad, rd::RAttrKey};

pub struct RBuffModifier {
    pub affectee_filter: ad::ABuffAffecteeFilter,
    pub affectee_attr_key: RAttrKey,
}
