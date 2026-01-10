use crate::num::PValue;

#[derive(Copy, Clone)]
pub struct Ecm {
    pub radar: PValue,
    pub magnetometric: PValue,
    pub gravimetric: PValue,
    pub ladar: PValue,
    pub duration: PValue,
}
