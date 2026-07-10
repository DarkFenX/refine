use crate::{
    dbg::DebugResult,
    svc::calc::{
        calce_rah::RahSim,
        misc::AttrValData,
        registers::{BuffRegister, DependencyRegister, RevisionRegister, StandardRegister},
    },
    ud::UData,
};

#[derive(Clone)]
pub(crate) struct Calc {
    pub(in crate::svc::calc) attrs: AttrValData,
    pub(in crate::svc::calc) std: StandardRegister,
    pub(in crate::svc::calc) buffs: BuffRegister,
    pub(in crate::svc::calc) deps: DependencyRegister,
    pub(in crate::svc::calc) revs: RevisionRegister,
    pub(in crate::svc::calc) rah: RahSim,
}
impl Calc {
    pub(in crate::svc) fn new() -> Self {
        Self {
            attrs: AttrValData::new(),
            std: StandardRegister::new(),
            buffs: BuffRegister::new(),
            deps: DependencyRegister::new(),
            revs: RevisionRegister::new(),
            rah: RahSim::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Calc {
    pub(in crate::svc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.attrs.consistency_check(u_data)?;
        self.std.consistency_check(u_data)?;
        self.buffs.consistency_check(u_data)?;
        self.deps.consistency_check(u_data)?;
        self.revs.consistency_check(u_data)?;
        self.rah.consistency_check(u_data)?;
        Ok(())
    }
}
