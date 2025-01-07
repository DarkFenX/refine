use crate::{
    sol::svc::calc::{
        calce_rah::SolRahSim,
        misc::SolAttrValData,
        registers::{
            SolBuffRegister, SolDependencyRegister, SolProjectionRegister, SolRevisionRegister, SolStandardRegister,
        },
    },
    src::Src,
};

#[derive(Clone)]
pub(in crate::sol) struct SolCalc {
    pub(in crate::sol::svc::calc) attrs: SolAttrValData,
    pub(in crate::sol::svc::calc) std: SolStandardRegister,
    pub(in crate::sol::svc::calc) buffs: SolBuffRegister,
    pub(in crate::sol::svc::calc) deps: SolDependencyRegister,
    pub(in crate::sol::svc::calc) revs: SolRevisionRegister,
    pub(in crate::sol::svc::calc) projs: SolProjectionRegister,
    pub(in crate::sol::svc::calc) rah: SolRahSim,
}
impl SolCalc {
    pub(in crate::sol::svc) fn new(src: &Src) -> Self {
        Self {
            attrs: SolAttrValData::new(),
            std: SolStandardRegister::new(),
            buffs: SolBuffRegister::new(),
            deps: SolDependencyRegister::new(),
            revs: SolRevisionRegister::new(),
            projs: SolProjectionRegister::new(),
            rah: SolRahSim::new(src),
        }
    }
}
