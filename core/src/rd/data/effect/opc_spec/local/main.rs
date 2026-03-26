use crate::{
    ad::AAttrId,
    nd::{NChargeMultGetter, NEffectLocalOpcSpec, NOutputGetter},
    rd::RAttrId,
    util::RMap,
};

#[derive(Copy, Clone)]
pub(crate) struct REffectLocalOpcSpec<BG>
where
    BG: NOutputGetter,
{
    pub(crate) base: BG,
    pub(crate) charge_mult: Option<NChargeMultGetter>,
    pub(crate) limit_attr_rid: Option<RAttrId>,
}
impl<BG> REffectLocalOpcSpec<BG>
where
    BG: NOutputGetter + Copy,
{
    pub(in crate::rd::data::effect) fn from_n_local_opc_spec(
        n_local_opc_spec: &NEffectLocalOpcSpec<BG>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Self {
        Self {
            base: n_local_opc_spec.base,
            charge_mult: n_local_opc_spec.charge_mult,
            limit_attr_rid: n_local_opc_spec
                .limit_attr_id
                .and_then(|v| attr_aid_rid_map.get(&v).copied()),
        }
    }
}
