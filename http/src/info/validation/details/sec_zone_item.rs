use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

use crate::shared::HSecZone;

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::validation) struct HValItemSecZoneFail {
    zone: HSecZone,
    #[serde_as(as = "&Map<DisplayFromStr, _>")]
    items: Vec<(rc::ItemId, Vec<HSecZone>)>,
}
impl From<&rc::val::ValItemSecZoneFail> for HValItemSecZoneFail {
    fn from(core_val_fail: &rc::val::ValItemSecZoneFail) -> Self {
        Self {
            zone: HSecZone::from_core(core_val_fail.zone),
            items: core_val_fail
                .items
                .iter()
                .map(|(item_id, allowed_sec_zones)| {
                    (
                        *item_id,
                        allowed_sec_zones.iter().map(|v| HSecZone::from_core(*v)).collect(),
                    )
                })
                .collect(),
        }
    }
}
