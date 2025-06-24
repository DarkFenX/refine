use super::{calc::Calc, eprojs::EProjs, vast::Vast};
use crate::src::Src;

#[derive(Clone)]
pub(in crate::sol) struct Svc {
    pub(in crate::sol::svc) calc: Calc,
    pub(in crate::sol::svc) vast: Vast,
    pub(in crate::sol::svc) eprojs: EProjs,
}
impl Svc {
    pub(in crate::sol) fn new(src: &Src) -> Self {
        Self {
            calc: Calc::new(src),
            vast: Vast::new(),
            eprojs: EProjs::new(),
        }
    }
}
