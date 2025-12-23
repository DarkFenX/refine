use crate::def::AttrVal;

#[derive(Copy, Clone)]
pub struct AttrVals {
    pub base: AttrVal,
    pub modified: AttrVal,
}
