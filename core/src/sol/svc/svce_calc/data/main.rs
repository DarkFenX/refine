use crate::sol::svc::svce_calc::{
    misc::SolAttrValData,
    registers::{
        SolAffecteeRegister, SolBuffRegister, SolDependencyRegister, SolModifierRegister, SolProjectionRegister,
        SolRevisionRegister,
    },
};

pub(in crate::sol::svc) struct SolSvcCalcData {
    pub(in crate::sol::svc::svce_calc) attrs: SolAttrValData,
    pub(in crate::sol::svc::svce_calc) mods: SolModifierRegister,
    pub(in crate::sol::svc::svce_calc) afee: SolAffecteeRegister,
    pub(in crate::sol::svc::svce_calc) buffs: SolBuffRegister,
    pub(in crate::sol::svc::svce_calc) deps: SolDependencyRegister,
    pub(in crate::sol::svc::svce_calc) revs: SolRevisionRegister,
    pub(in crate::sol::svc::svce_calc) projs: SolProjectionRegister,
}
impl SolSvcCalcData {
    pub(in crate::sol::svc) fn new() -> Self {
        Self {
            attrs: SolAttrValData::new(),
            mods: SolModifierRegister::new(),
            afee: SolAffecteeRegister::new(),
            buffs: SolBuffRegister::new(),
            deps: SolDependencyRegister::new(),
            revs: SolRevisionRegister::new(),
            projs: SolProjectionRegister::new(),
        }
    }
}
