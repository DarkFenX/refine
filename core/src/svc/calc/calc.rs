use crate::svc::calc::{
    calce_rah::RahSim,
    misc::AttrValData,
    registers::{BuffRegister, DependencyRegister, RevisionRegister, StandardRegister},
};

#[derive(Clone)]
pub(crate) struct Calc {
    pub(in crate::svc::calc) attrs: AttrValData,
    pub(in crate::svc::calc) std: StandardRegister,
    pub(in crate::svc::calc) buffs: BuffRegister,
    pub(crate) deps: DependencyRegister,
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
