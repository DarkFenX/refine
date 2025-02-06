use std::collections::HashMap;

use crate::shared::HModuleState;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HModuleStateValFail {
    #[serde(flatten)]
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    data: HashMap<rc::SolItemId, HModuleStateInfo>,
}
impl HModuleStateValFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::SolModuleStateValFail>> for HModuleStateValFail {
    fn from(core_val_fails: &Vec<rc::SolModuleStateValFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.item_id, v.into())).collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HModuleStateInfo {
    state: HModuleState,
    max_state: HModuleState,
}
impl From<&rc::SolModuleStateValFail> for HModuleStateInfo {
    fn from(core_val_fail: &rc::SolModuleStateValFail) -> Self {
        Self {
            state: (&core_val_fail.state).into(),
            max_state: (&core_val_fail.max_state).into(),
        }
    }
}
