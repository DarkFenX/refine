use crate::{ad::AAttrId, nd::NSpoolAttrs, rd::RAttrKey, util::RMap};

#[derive(Copy, Clone)]
pub(crate) struct RSpoolAttrs {
    pub(crate) step: RAttrKey,
    pub(crate) max: RAttrKey,
}
impl RSpoolAttrs {
    pub(in crate::rd::data::effect) fn try_from_n_spool_attrs(
        n_spool_attrs: &NSpoolAttrs,
        attr_id_key_map: &RMap<AAttrId, RAttrKey>,
    ) -> Option<Self> {
        Some(Self {
            step: attr_id_key_map.get(&n_spool_attrs.step).copied()?,
            max: attr_id_key_map.get(&n_spool_attrs.max).copied()?,
        })
    }
}
