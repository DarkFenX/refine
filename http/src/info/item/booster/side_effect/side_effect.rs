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
        item_id: &rc::ItemId,
        core_se_info: &rc::SideEffectInfo,
    ) -> Self {
        let chance = match core_sol.get_item_attr(item_id, &core_se_info.chance_attr_id) {
            Ok(val) => val.extra,
            // No attribute - declare it as 0% chance instead of hiding from info, to be consistent
            // with how effect runner behaves (it does not run effect if chance attr ID is defined
            // regardless of its value)
            Err(_) => rc::AttrVal::from(0.0),
        };
        let strength = match core_se_info.strength {
            Some(core_se_str) => HSideEffectStr::from_core_str(core_sol, item_id, &core_se_str),
            None => None,
        };
        Self::new(chance, core_se_info.status, strength)
    }
}
