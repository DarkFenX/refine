use crate::{
    ad::AAttrId,
    dbg::DebugResult,
    nd::{NEffectChargeMultGetter, NEffectOutputGetter, NEffectProjGetter, NEffectProjOpcSpec},
    rd::{RAttrId, REffectResist},
    ud::UData,
    util::RMap,
};

#[derive(Copy, Clone)]
pub(crate) struct REffectProjOpcSpec<BG>
where
    BG: NEffectOutputGetter,
{
    pub(crate) base: BG,
    pub(crate) charge_mult: Option<NEffectChargeMultGetter>,
    pub(crate) spoolable: bool,
    pub(crate) proj_mult_str: Option<NEffectProjGetter>,
    pub(crate) proj_mult_chance: Option<NEffectProjGetter>,
    pub(crate) resist: Option<REffectResist>,
    pub(crate) local_limit_attr_id: Option<RAttrId>,
    pub(crate) remote_limit_attr_id: Option<RAttrId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<BG> REffectProjOpcSpec<BG>
where
    BG: NEffectOutputGetter + Copy,
{
    pub(in crate::rd::data::effect) fn from_n_proj_opc_spec(
        n_proj_opc_spec: &NEffectProjOpcSpec<BG>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Self {
        Self {
            base: n_proj_opc_spec.base,
            charge_mult: n_proj_opc_spec.charge_mult,
            spoolable: n_proj_opc_spec.spoolable,
            proj_mult_str: n_proj_opc_spec.proj_mult_str,
            proj_mult_chance: n_proj_opc_spec.proj_mult_chance,
            resist: n_proj_opc_spec
                .resist
                .as_ref()
                .and_then(|n_resist| REffectResist::try_from_n_effect_resist(n_resist, attr_aid_rid_map)),
            local_limit_attr_id: n_proj_opc_spec
                .local_limit_attr_id
                .and_then(|attr_aid| attr_aid_rid_map.get(&attr_aid).copied()),
            remote_limit_attr_id: n_proj_opc_spec
                .remote_limit_attr_id
                .and_then(|attr_aid| attr_aid_rid_map.get(&attr_aid).copied()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<BG> REffectProjOpcSpec<BG>
where
    BG: NEffectOutputGetter,
{
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(resist) = &self.resist {
            resist.consistency_check(u_data)?;
        }
        if let Some(attr_rid) = &self.remote_limit_attr_id {
            attr_rid.consistency_check(u_data)?;
        }
        Ok(())
    }
}
