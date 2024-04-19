use crate::ss::svc::svce_calc::{
    misc::SsAttrValData,
    registers::{SsBuffRegister, SsDependencyRegister, SsModifierRegister, SsRevisionRegister, SsTargetRegister},
};

pub(in crate::ss::svc) struct SsSvcCalcData {
    pub(in crate::ss::svc::svce_calc) attrs: SsAttrValData,
    pub(in crate::ss::svc::svce_calc) mods: SsModifierRegister,
    pub(in crate::ss::svc::svce_calc) tgts: SsTargetRegister,
    pub(in crate::ss::svc::svce_calc) buffs: SsBuffRegister,
    pub(in crate::ss::svc::svce_calc) deps: SsDependencyRegister,
    pub(in crate::ss::svc::svce_calc) revs: SsRevisionRegister,
}
impl SsSvcCalcData {
    pub(in crate::ss::svc) fn new() -> Self {
        Self {
            attrs: SsAttrValData::new(),
            mods: SsModifierRegister::new(),
            tgts: SsTargetRegister::new(),
            buffs: SsBuffRegister::new(),
            deps: SsDependencyRegister::new(),
            revs: SsRevisionRegister::new(),
        }
    }
}
