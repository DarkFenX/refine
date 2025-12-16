use crate::{ad::AAttrId, ed::EAttrId, nd::NEffect};

impl NEffect {
    pub(in crate::ad::generator) fn extract_e_attr_ids(&self) -> Vec<EAttrId> {
        let mut e_attr_ids = Vec::new();
        if let Some(n_charge) = &self.charge
            && let Some(AAttrId::Eve(e_attr_id)) = n_charge.location.get_autocharge_attr_id()
        {
            e_attr_ids.push(e_attr_id)
        }
        e_attr_ids
    }
}
