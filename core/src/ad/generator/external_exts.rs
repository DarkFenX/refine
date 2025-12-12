use crate::{ad::AAttrId, nd::NEffect};

impl NEffect {
    pub(in crate::ad::generator) fn extract_a_attr_ids(&self) -> Vec<AAttrId> {
        let mut a_attr_ids = Vec::new();
        if let Some(n_charge) = &self.charge
            && let Some(a_attr_id) = n_charge.location.get_autocharge_attr_id()
        {
            a_attr_ids.push(a_attr_id)
        }
        a_attr_ids
    }
}
