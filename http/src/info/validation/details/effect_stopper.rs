use std::collections::HashMap;

use crate::shared::HEffectId;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValEffectStopperFail {
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    items: HashMap<rc::ItemId, Vec<HEffectId>>,
}
impl From<&rc::val::ValEffectStopperFail> for HValEffectStopperFail {
    fn from(core_val_fail: &rc::val::ValEffectStopperFail) -> Self {
        Self {
            items: core_val_fail
                .items
                .iter()
                .map(|(item_id, core_effect_ids)| (*item_id, core_effect_ids.iter().map(Into::into).collect()))
                .collect(),
        }
    }
}
