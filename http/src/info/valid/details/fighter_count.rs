use std::collections::HashMap;

use crate::shared::HModuleState;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HValFighterCountFail {
    #[serde(flatten)]
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    data: HashMap<rc::SolItemId, HValFighterCountItemInfo>,
}
impl HValFighterCountFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::SolValFighterCountFail>> for HValFighterCountFail {
    fn from(core_val_fails: &Vec<rc::SolValFighterCountFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.item_id, v.into())).collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValFighterCountItemInfo {
    count: rc::Count,
    max_count: rc::Count,
}
impl From<&rc::SolValFighterCountFail> for HValFighterCountItemInfo {
    fn from(core_val_fail: &rc::SolValFighterCountFail) -> Self {
        Self {
            count: core_val_fail.count,
            max_count: core_val_fail.max_count,
        }
    }
}
