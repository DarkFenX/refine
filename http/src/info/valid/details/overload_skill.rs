use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValOverloadSkillFail {
    td_lvl: Option<rc::SkillLevel>,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    module_reqs: HashMap<rc::ItemId, rc::SkillLevel>,
}
impl From<&rc::val::ValOverloadSkillFail> for HValOverloadSkillFail {
    fn from(core_val_fail: &rc::val::ValOverloadSkillFail) -> Self {
        Self {
            td_lvl: core_val_fail.td_lvl,
            module_reqs: core_val_fail.module_reqs.clone(),
        }
    }
}
