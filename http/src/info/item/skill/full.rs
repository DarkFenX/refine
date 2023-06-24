use std::collections::HashMap;

use crate::info::HAttrVal;

use super::HSkillInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HSkillInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HSkillInfoPartial,
    pub(crate) attr_vals: HashMap<rc::ReeInt, HAttrVal>,
}
impl HSkillInfoFull {
    pub(super) fn mk_info(core_ss: &mut rc::SolarSystem, core_skill_info: &rc::SsSkillInfo) -> Self {
        let partial_info = HSkillInfoPartial::from(core_skill_info);
        let attr_vals = match core_ss.get_item_attrs(&partial_info.id) {
            Ok(attrs) => attrs.into_iter().map(|(k, v)| (k, HAttrVal::from(&v))).collect(),
            _ => HashMap::new(),
        };
        Self {
            partial_info,
            attr_vals,
        }
    }
}
