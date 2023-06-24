use std::collections::HashMap;

use crate::info::HAttrVal;

use super::HSubsystemInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HSubsystemInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HSubsystemInfoPartial,
    pub(crate) attr_vals: HashMap<rc::ReeInt, HAttrVal>,
}
impl HSubsystemInfoFull {
    pub(super) fn mk_info(core_ss: &mut rc::SolarSystem, core_subsystem_info: &rc::SsSubsystemInfo) -> Self {
        let partial_info = HSubsystemInfoPartial::from(core_subsystem_info);
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
