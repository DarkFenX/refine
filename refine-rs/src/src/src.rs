use std::sync::Arc;

use crate::{refine::Refine, src::SrcAlias};

pub struct Src<'a> {
    pub(super) refine: &'a mut Refine,
    pub(super) alias: SrcAlias,
    core_src: Arc<rc::Src>,
}
impl<'a> Src<'a> {
    pub(super) fn new(refine: &'a mut Refine, alias: SrcAlias, core_src: Arc<rc::Src>) -> Self {
        Self {
            refine,
            alias,
            core_src,
        }
    }
}
