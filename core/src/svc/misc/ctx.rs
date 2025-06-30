use crate::{svc::eprojs::EProjs, uad::Uad};

pub(crate) struct SvcCtx<'u, 'p> {
    pub(crate) uad: &'u Uad,
    pub(in crate::svc) eprojs: &'p EProjs,
}
impl<'u, 'p> SvcCtx<'u, 'p> {
    pub(in crate::svc) fn new(uad: &'u Uad, eprojs: &'p EProjs) -> Self {
        Self { uad, eprojs }
    }
}
