use crate::{svc::eff_projs::EffProjs, ud::UData};

#[derive(Copy, Clone)]
pub(crate) struct SvcCtx<'u, 'p> {
    pub(crate) u_data: &'u UData,
    pub(crate) eff_projs: &'p EffProjs,
}
impl<'u, 'p> SvcCtx<'u, 'p> {
    pub(in crate::svc) fn new(u_data: &'u UData, eff_projs: &'p EffProjs) -> Self {
        Self {
            u_data: u_data,
            eff_projs,
        }
    }
}
