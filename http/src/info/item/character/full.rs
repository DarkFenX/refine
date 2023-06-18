use std::collections::HashMap;

use crate::info::HAttrVal;

use super::HCharacterInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HCharacterInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HCharacterInfoPartial,
    pub(crate) attr_vals: HashMap<rc::ReeInt, HAttrVal>,
}
impl HCharacterInfoFull {
    pub(super) fn mk_info(core_ss: &mut rc::SolarSystem, core_character_info: &rc::SsCharacterInfo) -> Self {
        let partial_info = HCharacterInfoPartial::from(core_character_info);
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
