use std::collections::HashMap;

use crate::info::HAttrVal;

use super::HRigInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HRigInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HRigInfoPartial,
    pub(crate) attr_vals: HashMap<rc::ReeInt, HAttrVal>,
}
impl HRigInfoFull {
    pub(super) fn mk_info(core_ss: &mut rc::SolarSystem, core_rig_info: &rc::SsRigInfo) -> Self {
        let partial_info = HRigInfoPartial::from(core_rig_info);
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
