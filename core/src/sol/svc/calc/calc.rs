use crate::{
    sol::svc::calc::{
        calce_rah::RahSim,
        misc::{AccumMgr, AttrValData},
        registers::{BuffRegister, DependencyRegister, ProjectionRegister, RevisionRegister, StandardRegister},
    },
    src::Src,
};

pub(in crate::sol) struct Calc {
    pub(in crate::sol::svc::calc) accums: AccumMgr,
    pub(in crate::sol::svc::calc) attrs: AttrValData,
    pub(in crate::sol::svc::calc) std: StandardRegister,
    pub(in crate::sol::svc::calc) buffs: BuffRegister,
    pub(in crate::sol::svc::calc) deps: DependencyRegister,
    pub(in crate::sol::svc::calc) revs: RevisionRegister,
    pub(in crate::sol::svc::calc) projs: ProjectionRegister,
    pub(in crate::sol::svc::calc) rah: RahSim,
}
impl Calc {
    pub(in crate::sol::svc) fn new(src: &Src) -> Self {
        Self {
            accums: AccumMgr::new(),
            attrs: AttrValData::new(),
            std: StandardRegister::new(),
            buffs: BuffRegister::new(),
            deps: DependencyRegister::new(),
            revs: RevisionRegister::new(),
            projs: ProjectionRegister::new(),
            rah: RahSim::new(src),
        }
    }
}
impl Clone for Calc {
    fn clone(&self) -> Self {
        Self {
            accums: AccumMgr::new(),
            attrs: self.attrs.clone(),
            std: self.std.clone(),
            buffs: self.buffs.clone(),
            deps: self.deps.clone(),
            revs: self.revs.clone(),
            projs: self.projs.clone(),
            rah: self.rah.clone(),
        }
    }
}
