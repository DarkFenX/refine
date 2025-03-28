use crate::sol::AttrVal;

#[derive(Copy, Clone)]
pub struct CalcAttrVal {
    pub base: AttrVal,
    pub dogma: AttrVal,
    pub extra: AttrVal,
}
