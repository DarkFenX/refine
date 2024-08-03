use super::HSideEffectStr;

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HSideEffectInfo {
    pub(crate) chance: rc::AttrVal,
    pub(crate) status: bool,
    pub(crate) strength: Option<HSideEffectStr>,
}
impl HSideEffectInfo {
    fn new(chance: rc::AttrVal, status: bool, strength: Option<HSideEffectStr>) -> Self {
        Self {
            chance,
            status,
            strength,
        }
    }
    pub(in crate::info::item::booster) fn from_core_info(
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::SolItemId,
        core_se_info: &rc::SolSideEffectInfo,
    ) -> Option<Self> {
        let chance = match core_sol.get_item_attr(item_id, &core_se_info.chance_attr_id) {
            Ok(val) => val.extra,
            _ => return None,
        };
        let strength = match core_se_info.strength {
            Some(core_se_str) => HSideEffectStr::from_core_str(core_sol, item_id, &core_se_str),
            None => None,
        };
        Some(Self::new(chance, core_se_info.status, strength))
    }
}
