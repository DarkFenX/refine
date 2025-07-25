use crate::{svc::eprojs::EProjs, ud::UData};

#[derive(Copy, Clone)]
pub(crate) struct SvcCtx<'u, 'p> {
    pub(crate) u_data: &'u UData,
    pub(crate) eprojs: &'p EProjs,
}
impl<'u, 'p> SvcCtx<'u, 'p> {
    pub(in crate::svc) fn new(u_data: &'u UData, eprojs: &'p EProjs) -> Self {
        Self { u_data: u_data, eprojs }
    }
}
