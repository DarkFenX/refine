use rc::ItemMutCommon;
use serde::Deserialize;
use serde_with::{DisplayFromStr, Map, serde_as};

use crate::shared::HEffectMode;

#[serde_as]
#[derive(Deserialize)]
#[serde(transparent)]
pub(in crate::cmd) struct HEffectModeMap {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    data: Vec<(rc::EffectId, HEffectMode)>,
}
impl HEffectModeMap {
    pub(in crate::cmd) fn apply(&self, core_item: &mut impl ItemMutCommon) {
        if !self.data.is_empty() {
            core_item.set_effect_modes(self.data.iter().map(|(k, v)| (*k, v.into_core())));
        }
    }
}
