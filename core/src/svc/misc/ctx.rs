use crate::{svc::eprojs::EProjs, uad::Uad};

#[derive(Copy, Clone)]
pub(crate) struct SvcCtx<'u, 'p> {
    pub(crate) uad: &'u Uad,
    pub(crate) eprojs: &'p EProjs,
}
impl<'u, 'p> SvcCtx<'u, 'p> {
    pub(in crate::svc) fn new(uad: &'u Uad, eprojs: &'p EProjs) -> Self {
        Self { uad, eprojs }
    }
}
