use std::collections::HashMap;

use crate::info::HAttrVal;

use super::HDroneInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HDroneInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HDroneInfoPartial,
    pub(crate) attr_vals: HashMap<rc::ReeInt, HAttrVal>,
}
impl HDroneInfoFull {
    pub(super) fn mk_info(core_ss: &mut rc::SolarSystem, core_drone_info: &rc::SsDroneInfo) -> Self {
        let partial_info = HDroneInfoPartial::from(core_drone_info);
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
