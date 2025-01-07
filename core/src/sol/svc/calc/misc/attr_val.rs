use crate::defs::AttrVal;

#[derive(Copy, Clone)]
pub struct SolAttrVal {
    pub base: AttrVal,
    pub dogma: AttrVal,
    pub extra: AttrVal,
}
impl SolAttrVal {
    pub(in crate::sol) fn new(base: AttrVal, dogma: AttrVal, extra: AttrVal) -> Self {
        Self { base, dogma, extra }
    }
}
