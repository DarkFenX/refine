use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::validation) struct HValOverloadSkillFail {
    td_lvl: Option<u8>,
    #[serde_as(as = "&Map<DisplayFromStr, _>")]
    module_reqs: Vec<(rc::ItemId, u8)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HValOverloadSkillFail {
    pub(in crate::info::validation) fn from_core(core_val_fail: rc::val::ValOverloadSkillFail) -> Self {
        Self {
            td_lvl: core_val_fail.td_lvl.map(|core_lvl| core_lvl.into_u8()),
            module_reqs: core_val_fail
                .module_reqs
                .into_iter()
                .map(|(k, v)| (k, v.into_u8()))
                .collect(),
        }
    }
}
