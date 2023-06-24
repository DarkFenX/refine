use std::collections::HashMap;

use crate::info::HAttrVal;

use super::HSwEffectInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HSwEffectInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HSwEffectInfoPartial,
    pub(crate) attr_vals: HashMap<rc::ReeInt, HAttrVal>,
}
impl HSwEffectInfoFull {
    pub(super) fn mk_info(core_ss: &mut rc::SolarSystem, core_sw_effect_info: &rc::SsSwEffectInfo) -> Self {
        let partial_info = HSwEffectInfoPartial::from(core_sw_effect_info);
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
