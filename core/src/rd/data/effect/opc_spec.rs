use crate::{
    ad::AAttrId,
    nd::{
        NBaseOutputGetter, NChargeMultGetter, NEffectLocalOpcSpec, NEffectProjOpcSpec, NProjMultGetter, NSpoolGetter,
    },
    rd::RAttrKey,
    util::RMap,
};

#[derive(Copy, Clone)]
pub(crate) struct REffectLocalOpcSpec<T>
where
    T: Copy
{
    pub(crate) base: NBaseOutputGetter<T>,
    pub(crate) charge_mult: Option<NChargeMultGetter> = None,
    pub(crate) ilimit_attr_key: Option<RAttrKey> = None,
}
impl<T> REffectLocalOpcSpec<T>
where
    T: Copy,
{
    pub(in crate::rd::data::effect) fn from_n_local_opc_spec(
        n_local_opc_spec: &NEffectLocalOpcSpec<T>,
        attr_id_key_map: &RMap<AAttrId, RAttrKey>,
    ) -> Self {
        Self {
            base: n_local_opc_spec.base,
            charge_mult: n_local_opc_spec.charge_mult,
            ilimit_attr_key: n_local_opc_spec
                .ilimit_attr_id
                .and_then(|v| attr_id_key_map.get(&v).copied()),
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct REffectProjOpcSpec<T>
where
    T: Copy
{
    pub(crate) base: NBaseOutputGetter<T>,
    pub(crate) proj_mult: NProjMultGetter,
    pub(crate) spool: Option<NSpoolGetter> = None,
    pub(crate) charge_mult: Option<NChargeMultGetter> = None,
    pub(crate) ilimit_attr_key: Option<RAttrKey> = None,
}
impl<T> REffectProjOpcSpec<T>
where
    T: Copy,
{
    pub(in crate::rd::data::effect) fn from_n_proj_opc_spec(
        n_proj_opc_spec: &NEffectProjOpcSpec<T>,
        attr_id_key_map: &RMap<AAttrId, RAttrKey>,
    ) -> Self {
        Self {
            base: n_proj_opc_spec.base,
            proj_mult: n_proj_opc_spec.proj_mult,
            spool: n_proj_opc_spec.spool,
            charge_mult: n_proj_opc_spec.charge_mult,
            ilimit_attr_key: n_proj_opc_spec
                .ilimit_attr_id
                .and_then(|v| attr_id_key_map.get(&v).copied()),
        }
    }
}
