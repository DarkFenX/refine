use std::collections::HashMap;

use crate::info::{HAttrVal, HEffect};

use super::HRigInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HRigInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HRigInfoPartial,
    pub(crate) attrs: HashMap<rc::AttrId, HAttrVal>,
    pub(crate) effects: HashMap<rc::EffectId, HEffect>,
}
impl HRigInfoFull {
    pub(super) fn mk_info(core_ss: &mut rc::SolarSystem, core_rig_info: &rc::SsRigInfo) -> Self {
        let partial_info = HRigInfoPartial::from(core_rig_info);
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
