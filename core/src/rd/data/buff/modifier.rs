use crate::{
    ad::{AAttrId, ABuffAffecteeFilter, ABuffModifier},
    rd::RAttrId,
    util::RMap,
};

pub(crate) struct RBuffModifier {
    pub(crate) affectee_filter: ABuffAffecteeFilter,
    pub(crate) affectee_attr_key: RAttrId,
}
impl RBuffModifier {
    pub(in crate::rd::data::buff) fn try_from_a_buff_mod(
        a_buff_mod: &ABuffModifier,
        attr_id_key_map: &RMap<AAttrId, RAttrId>,
    ) -> Option<Self> {
        let affectee_attr_key = *attr_id_key_map.get(&a_buff_mod.affectee_attr_id)?;
        Some(Self {
            affectee_filter: a_buff_mod.affectee_filter,
            affectee_attr_key,
        })
    }
}
