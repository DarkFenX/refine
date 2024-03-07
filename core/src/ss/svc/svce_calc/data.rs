use super::misc::{AttrCapData, AttrValData, ModRegister, ProjRegister, RevisionRegister};

pub(in crate::ss::svc) struct CalcData {
    pub(in crate::ss::svc::svce_calc) attrs: AttrValData,
    pub(in crate::ss::svc::svce_calc) caps: AttrCapData,
    pub(in crate::ss::svc::svce_calc) mods: ModRegister,
    pub(in crate::ss::svc::svce_calc) projs: ProjRegister,
    pub(in crate::ss::svc::svce_calc) revs: RevisionRegister,
}
impl CalcData {
    pub(in crate::ss::svc) fn new() -> Self {
        Self {
            attrs: AttrValData::new(),
            caps: AttrCapData::new(),
            mods: ModRegister::new(),
            projs: ProjRegister::new(),
            revs: RevisionRegister::new(),
        }
    }
}
