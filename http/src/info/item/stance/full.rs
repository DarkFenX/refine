use std::collections::HashMap;

use crate::info::HAttrVal;

use super::HStanceInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HStanceInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HStanceInfoPartial,
    pub(crate) attr_vals: HashMap<rc::ReeInt, HAttrVal>,
}
impl HStanceInfoFull {
    pub(super) fn mk_info(core_ss: &mut rc::SolarSystem, core_stance_info: &rc::SsStanceInfo) -> Self {
        let partial_info = HStanceInfoPartial::from(core_stance_info);
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
