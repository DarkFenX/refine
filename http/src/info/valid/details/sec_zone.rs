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
                .map(|i| (i.item_id, i.allowed_zones.iter().map(|z| z.into()).collect()))
                .collect(),
        }
    }
}
