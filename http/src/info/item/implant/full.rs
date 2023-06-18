use std::collections::HashMap;

use crate::info::HAttrVal;

use super::HImplantInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HImplantInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HImplantInfoPartial,
    pub(crate) attr_vals: HashMap<rc::ReeInt, HAttrVal>,
}
impl HImplantInfoFull {
    pub(super) fn mk_info(core_ss: &mut rc::SolarSystem, core_implant_info: &rc::SsImplantInfo) -> Self {
        let partial_info = HImplantInfoPartial::from(core_implant_info);
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
