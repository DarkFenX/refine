use super::{calc::Calc, eff_projs::EffProjs, vast::Vast};

#[derive(Clone)]
pub(crate) struct Svc {
    pub(in crate::svc) calc: Calc,
    pub(in crate::svc) vast: Vast,
    pub(in crate::svc) eff_projs: EffProjs,
}
impl Svc {
    pub(crate) fn new() -> Self {
        Self {
            calc: Calc::new(),
            vast: Vast::new(),
            eff_projs: EffProjs::new(),
        }
    }
}
