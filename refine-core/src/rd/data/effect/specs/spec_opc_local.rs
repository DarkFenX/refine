use crate::{
    ad::AAttrId,
    dbg::DebugResult,
    nd::{NEffectChargeMultGetter, NEffectLocalOpcSpec, NEffectOutputGetter},
    rd::RAttrId,
    ud::UData,
    util::RMap,
};

#[derive(Copy, Clone)]
pub(crate) struct REffectLocalOpcSpec<BG>
where
    BG: NEffectOutputGetter,
{
    pub(crate) base: BG,
    pub(crate) charge_mult: Option<NEffectChargeMultGetter>,
    pub(crate) limit_attr_rid: Option<RAttrId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<BG> REffectLocalOpcSpec<BG>
where
    BG: NEffectOutputGetter + Copy,
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<BG> REffectLocalOpcSpec<BG>
where
    BG: NEffectOutputGetter,
{
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(attr_rid) = self.limit_attr_rid {
            attr_rid.consistency_check(u_data)?;
        }
        Ok(())
    }
}
