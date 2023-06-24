use std::collections::HashMap;

use crate::info::HAttrVal;

use super::HBoosterInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HBoosterInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HBoosterInfoPartial,
    pub(crate) attr_vals: HashMap<rc::ReeInt, HAttrVal>,
}
impl HBoosterInfoFull {
    pub(super) fn mk_info(core_ss: &mut rc::SolarSystem, core_booster_info: &rc::SsBoosterInfo) -> Self {
        let partial_info = HBoosterInfoPartial::from(core_booster_info);
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
