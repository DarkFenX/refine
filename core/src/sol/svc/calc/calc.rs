use crate::{
    sol::svc::calc::{
        calce_rah::RahSim,
        misc::AttrValData,
        registers::{BuffRegister, DependencyRegister, ProjectionRegister, RevisionRegister, StandardRegister},
    },
    src::Src,
};

#[derive(Clone)]
pub(in crate::sol) struct Calc {
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
