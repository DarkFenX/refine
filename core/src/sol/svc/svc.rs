use super::{calc::Calc, vast::Vast};
use crate::src::Src;

#[derive(Clone)]
pub(in crate::sol) struct Svc {
    pub(in crate::sol) calc: Calc,
    pub(in crate::sol) vast: Vast,
}
impl Svc {
    pub(in crate::sol) fn new(src: &Src) -> Self {
        Self {
            calc: Calc::new(src),
            vast: Vast::new(),
        }
    }
}
