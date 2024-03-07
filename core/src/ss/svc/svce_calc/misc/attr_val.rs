use crate::defs::AttrVal;

#[derive(Copy, Clone)]
pub struct SsAttrVal {
    pub base: AttrVal,
    pub dogma: AttrVal,
    pub extra: AttrVal,
}
impl SsAttrVal {
    pub(in crate::ss) fn new(base: AttrVal, dogma: AttrVal, extra: AttrVal) -> Self {
        Self { base, dogma, extra }
    }
}
