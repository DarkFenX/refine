use std::collections::HashMap;

use crate::info::HAttrVal;

use super::HChargeInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HChargeInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HChargeInfoPartial,
    pub(crate) attr_vals: HashMap<rc::ReeInt, HAttrVal>,
}
impl HChargeInfoFull {
    pub(super) fn mk_info(core_ss: &mut rc::SolarSystem, core_charge_info: &rc::SsChargeInfo) -> Self {
        let partial_info = HChargeInfoPartial::from(core_charge_info);
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
