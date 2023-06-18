use std::collections::HashMap;

use crate::info::HAttrVal;

use super::HShipInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HShipInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HShipInfoPartial,
    pub(crate) attr_vals: HashMap<rc::ReeInt, HAttrVal>,
}
impl HShipInfoFull {
    pub(super) fn mk_info(core_ss: &mut rc::SolarSystem, core_ship_info: &rc::SsShipInfo) -> Self {
        let partial_info = HShipInfoPartial::from(core_ship_info);
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
