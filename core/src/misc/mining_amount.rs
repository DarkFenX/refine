use crate::def::AttrVal;

#[derive(Copy, Clone)]
pub struct MiningAmount {
    pub yield_: AttrVal,
    pub drain: AttrVal,
}
impl Default for MiningAmount {
    fn default() -> Self {
        Self {
            yield_: AttrVal::default(),
            drain: AttrVal::default(),
        }
    }
}
