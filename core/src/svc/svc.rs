use super::{calc::Calc, eprojs::EProjs, vast::Vast};
use crate::src::Src;

#[derive(Clone)]
pub(crate) struct Svc {
    pub(in crate::svc) calc: Calc,
    pub(in crate::svc) vast: Vast,
    pub(in crate::svc) eprojs: EProjs,
}
impl Svc {
    pub(crate) fn new(src: &Src) -> Self {
        Self {
            calc: Calc::new(src),
            vast: Vast::new(),
            eprojs: EProjs::new(),
        }
    }
}
