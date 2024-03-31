use super::{
    misc::AttrValData,
    registers::{DependencyRegister, ModifierRegister, RevisionRegister, TargetRegister},
};

pub(in crate::ss::svc) struct CalcData {
    pub(in crate::ss::svc::svce_calc) attrs: AttrValData,
    pub(in crate::ss::svc::svce_calc) mods: ModifierRegister,
    pub(in crate::ss::svc::svce_calc) tgts: TargetRegister,
    pub(in crate::ss::svc::svce_calc) deps: DependencyRegister,
    pub(in crate::ss::svc::svce_calc) revs: RevisionRegister,
}
impl CalcData {
    pub(in crate::ss::svc) fn new() -> Self {
        Self {
            attrs: AttrValData::new(),
            mods: ModifierRegister::new(),
            tgts: TargetRegister::new(),
            deps: DependencyRegister::new(),
            revs: RevisionRegister::new(),
        }
    }
}
