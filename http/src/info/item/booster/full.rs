use std::collections::HashMap;

use crate::info::{HAttrVal, HEffect};

use super::HBoosterInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HBoosterInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HBoosterInfoPartial,
    pub(crate) attrs: HashMap<rc::EAttrId, HAttrVal>,
    pub(crate) effects: HashMap<rc::EEffectId, HEffect>,
}
impl HBoosterInfoFull {
    pub(super) fn mk_info(core_ss: &mut rc::SolarSystem, core_booster_info: &rc::SsBoosterInfo) -> Self {
        let partial_info = HBoosterInfoPartial::from(core_booster_info);
        let attrs = match core_ss.get_item_attrs(&partial_info.id) {
            Ok(core_attrs) => core_attrs.into_iter().map(|(k, v)| (k, HAttrVal::from(&v))).collect(),
            _ => HashMap::new(),
        };
        let effects = match core_ss.get_item_effects(&partial_info.id) {
            Ok(core_effects) => core_effects.into_iter().map(|(k, v)| (k, HEffect::from(&v))).collect(),
            _ => HashMap::new(),
        };
        Self {
            partial_info,
            attrs,
            effects,
        }
    }
}
