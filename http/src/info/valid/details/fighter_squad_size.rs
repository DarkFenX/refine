use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HValFighterSquadSizeFail {
    #[serde(flatten)]
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    data: HashMap<rc::SolItemId, HValFighterSquadSizeItemInfo>,
}
impl HValFighterSquadSizeFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::SolValFighterSquadSizeFail>> for HValFighterSquadSizeFail {
    fn from(core_val_fails: &Vec<rc::SolValFighterSquadSizeFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.item_id, v.into())).collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValFighterSquadSizeItemInfo {
    size: rc::Count,
    max_size: rc::Count,
}
impl From<&rc::SolValFighterSquadSizeFail> for HValFighterSquadSizeItemInfo {
    fn from(core_val_fail: &rc::SolValFighterSquadSizeFail) -> Self {
        Self {
            size: core_val_fail.size,
            max_size: core_val_fail.max_size,
        }
    }
}
