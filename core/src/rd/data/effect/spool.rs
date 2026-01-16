use crate::{ad::AAttrId, nd::NEffectSpoolAttrs, rd::RAttrId, util::RMap};

#[derive(Copy, Clone)]
pub(crate) struct REffectSpoolAttrs {
    pub(crate) step_attr_rid: RAttrId,
    pub(crate) max_attr_rid: RAttrId,
}
impl REffectSpoolAttrs {
    pub(in crate::rd::data::effect) fn try_from_n_spool_attrs(
        n_spool_attrs: &NEffectSpoolAttrs,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Option<Self> {
        Some(Self {
            step_attr_rid: attr_aid_rid_map.get(&n_spool_attrs.step_attr_id).copied()?,
            max_attr_rid: attr_aid_rid_map.get(&n_spool_attrs.max_attr_id).copied()?,
        })
    }
}
