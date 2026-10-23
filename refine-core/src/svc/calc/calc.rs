use crate::{
    dbg::DebugResult,
    svc::calc::{
        calce_rah::RahSim,
        misc::{AttrValData, CalcReuse},
        registers::{BuffRegister, DependencyRegister, RawModStorage, RevisionRegister, StandardRegister},
    },
    ud::UData,
};

#[derive(Clone)]
pub(crate) struct Calc {
    pub(in crate::svc::calc) reuse: CalcReuse,
    pub(in crate::svc::calc) attrs: AttrValData,
    pub(in crate::svc::calc) rmods: RawModStorage,
    pub(in crate::svc::calc) std: StandardRegister,
    pub(in crate::svc::calc) buffs: BuffRegister,
    pub(in crate::svc::calc) deps: DependencyRegister,
    pub(in crate::svc::calc) revs: RevisionRegister,
    pub(in crate::svc::calc) rah: RahSim,
}
impl Calc {
    pub(in crate::svc) fn new() -> Self {
        Self {
            reuse: CalcReuse::default(),
            attrs: AttrValData::new(),
            rmods: RawModStorage::new(),
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
        let Self {
            attrs,
            rmods,
            std,
            buffs,
            deps,
            revs,
            rah,
            // Scratch containers
            reuse: _,
        } = self;
        attrs.consistency_check(u_data)?;
        rmods.consistency_check(u_data)?;
        std.consistency_check(u_data, rmods)?;
        buffs.consistency_check(u_data, rmods)?;
        deps.consistency_check(u_data)?;
        revs.consistency_check(u_data, rmods)?;
        rah.consistency_check(u_data)?;
        Ok(())
    }
}
