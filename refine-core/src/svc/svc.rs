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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl Svc {
        pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            let Self { calc, vast, eff_projs } = self;
            calc.consistency_check(u_data)?;
            vast.consistency_check(u_data)?;
            eff_projs.consistency_check(u_data)?;
            Ok(())
        }
    }
}
