use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValOverloadSkillFail {
    td_lvl: Option<rc::SkillLevel>,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    items: HashMap<rc::ItemId, rc::SkillLevel>,
}
impl From<&rc::val::ValOverloadSkillFail> for HValOverloadSkillFail {
    fn from(core_val_fail: &rc::val::ValOverloadSkillFail) -> Self {
        Self {
            td_lvl: core_val_fail.td_lvl,
            items: core_val_fail.items.iter().map(|v| (v.item_id, v.req_lvl)).collect(),
        }
    }
}
