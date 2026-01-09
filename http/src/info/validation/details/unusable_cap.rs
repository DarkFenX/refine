use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::validation) struct HValUnusableCapFail {
    max_cap: f64,
    #[serde_as(as = "&Map<DisplayFromStr, _>")]
    items: Vec<(rc::ItemId, f64)>,
}
impl From<&rc::val::ValUnusableCapFail> for HValUnusableCapFail {
    fn from(core_val_fail: &rc::val::ValUnusableCapFail) -> Self {
        Self {
            max_cap: core_val_fail.max_cap.into_f64(),
            items: core_val_fail.items.iter().map(|(k, v)| (*k, v.into_f64())).collect(),
        }
    }
}
