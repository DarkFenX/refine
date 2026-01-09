use serde::Serialize;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValEffectStopperFail {
    #[serde_as(as = "Map<DisplayFromStr, Vec<DisplayFromStr>>")]
    items: Vec<(rc::ItemId, Vec<rc::EffectId>)>,
}
impl From<&rc::val::ValEffectStopperFail> for HValEffectStopperFail {
    fn from(core_val_fail: &rc::val::ValEffectStopperFail) -> Self {
        Self {
            items: core_val_fail
                .items
                .iter()
                .map(|(item_id, core_effect_ids)| (*item_id, core_effect_ids.iter().copied().collect()))
                .collect(),
        }
    }
}
