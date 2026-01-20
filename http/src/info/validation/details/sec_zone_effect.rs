use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

use crate::shared::HSecZone;

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::validation) struct HValEffectSecZoneFail {
    zone: HSecZone,
    #[serde_as(as = "&Map<DisplayFromStr, Map<DisplayFromStr, _>>")]
    items: Vec<(rc::ItemId, Vec<(rc::EffectId, Vec<HSecZone>)>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HValEffectSecZoneFail {
    pub(in crate::info::validation) fn from_core(core_val_fail: rc::val::ValEffectSecZoneFail) -> Self {
        Self {
            zone: HSecZone::from_core(core_val_fail.zone),
            items: core_val_fail
                .items
                .into_iter()
                .map(|(item_id, item_data)| {
                    (
                        item_id,
                        item_data
                            .into_iter()
                            .map(|(effect_id, allowed_sec_zones)| {
                                (
                                    effect_id,
                                    allowed_sec_zones.into_iter().map(|v| HSecZone::from_core(v)).collect(),
                                )
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}
