use crate::{
    rd::{RAttrConsts, REffectConsts},
    svc::eff_projs::EffProjs,
    ud::UData,
};

#[derive(Copy, Clone)]
pub(crate) struct SvcCtx<'u, 'p> {
    pub(crate) u_data: &'u UData,
    pub(crate) eff_projs: &'p EffProjs,
}
impl<'u, 'p> SvcCtx<'u, 'p> {
    pub(in crate::svc) fn new(u_data: &'u UData, eff_projs: &'p EffProjs) -> Self {
        Self { u_data, eff_projs }
    }
    pub(crate) fn ac(&self) -> &RAttrConsts {
        self.u_data.src.get_attr_consts()
    }
    pub(crate) fn ec(&self) -> &REffectConsts {
        self.u_data.src.get_effect_consts()
    }
}
