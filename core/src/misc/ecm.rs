use crate::def::AttrVal;

#[derive(Copy, Clone)]
pub struct Ecm {
    pub radar: AttrVal,
    pub magnetometric: AttrVal,
    pub gravimetric: AttrVal,
    pub ladar: AttrVal,
    pub duration: AttrVal,
}
