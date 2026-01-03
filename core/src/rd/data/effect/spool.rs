use crate::{ad::AAttrId, nd::NSpoolAttrs, rd::RAttrId, util::RMap};

#[derive(Copy, Clone)]
pub(crate) struct RSpoolAttrs {
    pub(crate) step: RAttrId,
    pub(crate) max: RAttrId,
}
impl RSpoolAttrs {
    pub(in crate::rd::data::effect) fn try_from_n_spool_attrs(
        n_spool_attrs: &NSpoolAttrs,
        attr_id_key_map: &RMap<AAttrId, RAttrId>,
    ) -> Option<Self> {
        Some(Self {
            step: attr_id_key_map.get(&n_spool_attrs.step_attr_id).copied()?,
            max: attr_id_key_map.get(&n_spool_attrs.max_attr_id).copied()?,
        })
    }
}
