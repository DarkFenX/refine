use crate::defs::{EAttrId, SsItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::ss::svc::svce_calc) struct AttrSpec {
    item: SsItemId,
    attr: EAttrId,
}
impl AttrSpec {
    pub(super) fn new(item: SsItemId, attr: EAttrId) -> Self {
        Self { item, attr }
    }
}
