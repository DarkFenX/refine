use std::collections::HashMap;

use crate::info::{HAttrVal, HItemInfoMode};

use super::HModuleInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HModuleInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HModuleInfoPartial,
    pub(crate) attr_vals: HashMap<rc::ReeInt, HAttrVal>,
}
impl HModuleInfoFull {
    pub(super) fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_module_info: &rc::SsModuleInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        let partial_info = HModuleInfoPartial::mk_info(core_ss, core_module_info, item_mode);
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
