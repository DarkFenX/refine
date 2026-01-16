use crate::{
    ad::AAttrId,
    nd::{NBaseOutputGetter, NChargeMultGetter, NEffectProjMultGetter, NEffectProjOpcSpec},
    rd::{RAttrId, REffectResist},
    util::RMap,
};

#[derive(Copy, Clone)]
pub(crate) struct REffectProjOpcSpec<T>
where
    T: Copy,
{
    pub(crate) base: NBaseOutputGetter<T>,
    pub(crate) charge_mult: Option<NChargeMultGetter>,
    pub(crate) spoolable: bool,
    pub(crate) proj_mult_str: Option<NEffectProjMultGetter>,
    pub(crate) proj_mult_chance: Option<NEffectProjMultGetter>,
    pub(crate) resist: Option<REffectResist>,
    pub(crate) limit_attr_rid: Option<RAttrId>,
}
impl<T> REffectProjOpcSpec<T>
where
    T: Copy,
{
    pub(in crate::rd::data::effect) fn from_n_proj_opc_spec(
        n_proj_opc_spec: &NEffectProjOpcSpec<T>,
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
            limit_attr_rid: n_proj_opc_spec
                .limit_attr_id
                .and_then(|attr_aid| attr_aid_rid_map.get(&attr_aid).copied()),
        }
    }
}
