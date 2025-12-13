use std::collections::HashMap;

use crate::shared::{HEffectId, HSecZone};

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::validation) struct HValEffectSecZoneFail {
    zone: HSecZone,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    items: HashMap<rc::ItemId, HashMap<HEffectId, Vec<HSecZone>>>,
}
impl From<&rc::val::ValEffectSecZoneFail> for HValEffectSecZoneFail {
    fn from(core_val_fail: &rc::val::ValEffectSecZoneFail) -> Self {
        Self {
            zone: (&core_val_fail.zone).into(),
            items: core_val_fail
                .items
                .iter()
                .map(|(item_id, item_data)| {
                    (
                        *item_id,
                        item_data
                            .iter()
                            .map(|(effect_id, allowed_sec_zones)| {
                                (effect_id.into(), allowed_sec_zones.iter().map(Into::into).collect())
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}
