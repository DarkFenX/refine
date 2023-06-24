use std::collections::HashMap;

use crate::info::HAttrVal;

use super::HFighterInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HFighterInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HFighterInfoPartial,
    pub(crate) attr_vals: HashMap<rc::ReeInt, HAttrVal>,
}
impl HFighterInfoFull {
    pub(super) fn mk_info(core_ss: &mut rc::SolarSystem, core_fighter_info: &rc::SsFighterInfo) -> Self {
        let partial_info = HFighterInfoPartial::from(core_fighter_info);
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
