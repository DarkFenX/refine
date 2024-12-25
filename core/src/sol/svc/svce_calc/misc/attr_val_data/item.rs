use crate::{defs::EAttrId, sol::svc::svce_calc::SolAttrVal, util::StMap};

#[derive(Clone)]
pub(super) struct SolItemAttrValData {
    pub(super) values: StMap<EAttrId, SolAttrVal>,
    pub(super) overrides: StMap<EAttrId, SolAttrVal>,
}
impl SolItemAttrValData {
    pub(super) fn new() -> Self {
        Self {
            values: StMap::new(),
            overrides: StMap::new(),
        }
    }
    pub(super) fn get_attrs(&self) -> &StMap<EAttrId, SolAttrVal> {
        &self.values
    }
    pub(super) fn get_attrs_mut(&mut self) -> &mut StMap<EAttrId, SolAttrVal> {
        &mut self.values
    }
}
