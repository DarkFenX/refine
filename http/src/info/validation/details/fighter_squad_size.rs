use serde::Serialize;
use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValFighterSquadSizeFail {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    fighters: Vec<(rc::ItemId, HValFighterSquadSizeFighterInfo)>,
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

#[serde_as]
#[derive(Serialize_tuple)]
struct HValFighterSquadSizeFighterInfo {
    size: u32,
    max_size: u32,
}
impl From<&rc::val::ValFighterSquadSizeFighterInfo> for HValFighterSquadSizeFighterInfo {
    fn from(core_val_fighter_info: &rc::val::ValFighterSquadSizeFighterInfo) -> Self {
        Self {
            size: core_val_fighter_info.size.into_u32(),
            max_size: core_val_fighter_info.max_size.into_u32(),
        }
    }
}
