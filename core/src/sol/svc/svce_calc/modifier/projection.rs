use crate::defs::{EAttrId, SolItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::svce_calc) struct SolModProjInfo {
    pub(in crate::sol::svc::svce_calc) item_id: SolItemId,
    pub(in crate::sol::svc::svce_calc) resist_attr_id: Option<EAttrId>,
    pub(in crate::sol::svc::svce_calc) optimal_attr_id: Option<EAttrId>,
    pub(in crate::sol::svc::svce_calc) falloff_attr_id: Option<EAttrId>,
}
impl SolModProjInfo {
    pub(in crate::sol::svc::svce_calc) fn new(
        item_id: SolItemId,
        resist_attr_id: Option<EAttrId>,
        optimal_attr_id: Option<EAttrId>,
        falloff_attr_id: Option<EAttrId>,
    ) -> Self {
        Self {
            item_id,
            resist_attr_id,
            optimal_attr_id,
            falloff_attr_id,
        }
    }
}
