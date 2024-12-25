use crate::{
    defs::EAttrId,
    sol::{item::SolItem, svc::svce_calc::SolAttrVal, SolView},
    util::StMap,
};

#[derive(Clone)]
pub(in crate::sol::svc::svce_calc) struct SolItemAttrValData {
    pub(in crate::sol::svc::svce_calc) values: StMap<EAttrId, SolAttrVal>,
    pub(in crate::sol::svc::svce_calc) overrides: StMap<EAttrId, fn(&SolView, &SolItem) -> Option<SolAttrVal>>,
}
impl SolItemAttrValData {
    pub(super) fn new() -> Self {
        Self {
            values: StMap::new(),
            overrides: StMap::new(),
        }
    }
}
