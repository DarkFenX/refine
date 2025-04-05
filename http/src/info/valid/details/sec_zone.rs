use std::collections::HashMap;

use crate::shared::HSecZone;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValSecZoneFail {
    zone: HSecZone,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    items: HashMap<rc::ItemId, Vec<HSecZone>>,
}
impl From<&rc::val::ValSecZoneFail> for HValSecZoneFail {
    fn from(core_val_fail: &rc::val::ValSecZoneFail) -> Self {
        Self {
            zone: (&core_val_fail.zone).into(),
            items: core_val_fail
                .items
                .iter()
                .map(|(item_id, allowed_sec_zones)| {
                    (
                        *item_id,
                        allowed_sec_zones.iter().map(|sec_zone| sec_zone.into()).collect(),
                    )
                })
                .collect(),
        }
    }
}
