use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::val) struct HValFighterSquadSizeFail {
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    fighters: HashMap<rc::ItemId, HValFighterSquadSizeFighterInfo>,
}
impl From<&rc::val::ValFighterSquadSizeFail> for HValFighterSquadSizeFail {
    fn from(core_val_fail: &rc::val::ValFighterSquadSizeFail) -> Self {
        Self {
            fighters: core_val_fail
                .fighters
                .iter()
                .map(|(item_id, core_fighter_info)| (*item_id, core_fighter_info.into()))
                .collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
struct HValFighterSquadSizeFighterInfo {
    size: rc::Count,
    max_size: rc::Count,
}
impl From<&rc::val::ValFighterSquadSizeFighterInfo> for HValFighterSquadSizeFighterInfo {
    fn from(core_val_fighter_info: &rc::val::ValFighterSquadSizeFighterInfo) -> Self {
        Self {
            size: core_val_fighter_info.size,
            max_size: core_val_fighter_info.max_size,
        }
    }
}
