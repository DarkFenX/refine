use crate::{
    defs::EAttrId,
    sol::{item::SolItem, svc::svce_calc::SolAttrVal, SolView},
    util::StMap,
};

#[derive(Clone)]
pub(super) struct SolItemAttrValData {
    pub(super) values: StMap<EAttrId, SolAttrVal>,
    pub(super) overrides: StMap<EAttrId, fn(&SolView, &SolItem) -> Option<SolAttrVal>>,
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
