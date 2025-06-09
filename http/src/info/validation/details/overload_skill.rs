use std::collections::HashMap;

use crate::shared::HSkillLevel;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::validation) struct HValOverloadSkillFail {
    td_lvl: Option<HSkillLevel>,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    module_reqs: HashMap<rc::ItemId, HSkillLevel>,
}
impl From<&rc::val::ValOverloadSkillFail> for HValOverloadSkillFail {
    fn from(core_val_fail: &rc::val::ValOverloadSkillFail) -> Self {
        Self {
            td_lvl: core_val_fail.td_lvl.map(|core_lvl| core_lvl.get_inner()),
            module_reqs: core_val_fail
                .module_reqs
                .iter()
                .map(|(k, v)| (*k, v.get_inner()))
                .collect(),
        }
    }
}
