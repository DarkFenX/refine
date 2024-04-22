use crate::ss::svc::svce_calc::{
    misc::SsAttrValData,
    registers::{SolAffecteeRegister, SsBuffRegister, SsDependencyRegister, SsModifierRegister, SsRevisionRegister},
};

pub(in crate::ss::svc) struct SsSvcCalcData {
    pub(in crate::ss::svc::svce_calc) attrs: SsAttrValData,
    pub(in crate::ss::svc::svce_calc) mods: SsModifierRegister,
    pub(in crate::ss::svc::svce_calc) affectee: SolAffecteeRegister,
    pub(in crate::ss::svc::svce_calc) buffs: SsBuffRegister,
    pub(in crate::ss::svc::svce_calc) deps: SsDependencyRegister,
    pub(in crate::ss::svc::svce_calc) revs: SsRevisionRegister,
}
impl SsSvcCalcData {
    pub(in crate::ss::svc) fn new() -> Self {
        Self {
            attrs: SsAttrValData::new(),
            mods: SsModifierRegister::new(),
            affectee: SolAffecteeRegister::new(),
            buffs: SsBuffRegister::new(),
            deps: SsDependencyRegister::new(),
            revs: SsRevisionRegister::new(),
        }
    }
}
