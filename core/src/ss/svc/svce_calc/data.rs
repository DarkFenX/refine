use super::{
    misc::{AttrCapData, AttrValData},
    registers::{ModRegister, ProjectionRegister, RevisionRegister},
};

pub(in crate::ss::svc) struct CalcData {
    pub(in crate::ss::svc::svce_calc) attrs: AttrValData,
    pub(in crate::ss::svc::svce_calc) caps: AttrCapData,
    pub(in crate::ss::svc::svce_calc) mods: ModRegister,
    pub(in crate::ss::svc::svce_calc) projs: ProjectionRegister,
    pub(in crate::ss::svc::svce_calc) revs: RevisionRegister,
}
impl CalcData {
    pub(in crate::ss::svc) fn new() -> Self {
        Self {
            attrs: AttrValData::new(),
            caps: AttrCapData::new(),
            mods: ModRegister::new(),
            projs: ProjectionRegister::new(),
            revs: RevisionRegister::new(),
        }
    }
}
