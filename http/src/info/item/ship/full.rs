use std::collections::HashMap;

use crate::info::{HAttrVal, HEffect};

use super::HShipInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HShipInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HShipInfoPartial,
    pub(crate) attrs: HashMap<rc::AttrId, HAttrVal>,
    pub(crate) effects: HashMap<rc::EffectId, HEffect>,
}
impl HShipInfoFull {
    pub(super) fn mk_info(core_ss: &mut rc::SolarSystem, core_ship_info: &rc::SsShipInfo) -> Self {
        let partial_info = HShipInfoPartial::from(core_ship_info);
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
