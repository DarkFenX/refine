use crate::sol::AttrVal;

#[derive(Copy, Clone)]
pub struct CalcAttrVal {
    pub base: AttrVal,
    pub dogma: AttrVal,
    pub extra: AttrVal,
}
impl CalcAttrVal {
    pub(in crate::sol) fn new(base: AttrVal, dogma: AttrVal, extra: AttrVal) -> Self {
        Self { base, dogma, extra }
    }
}
