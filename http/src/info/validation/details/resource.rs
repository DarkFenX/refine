use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::validation) struct HValResFail {
    used: f64,
    max: Option<f64>,
    #[serde_as(as = "&Map<DisplayFromStr, _>")]
    users: Vec<(rc::ItemId, f64)>,
}
impl From<&rc::val::ValResFail> for HValResFail {
    fn from(core_val_fail: &rc::val::ValResFail) -> Self {
        Self {
            used: core_val_fail.used.into_f64(),
            max: core_val_fail.max.map(|v| v.into_f64()),
            users: core_val_fail.users.iter().map(|(k, v)| (*k, v.into_f64())).collect(),
        }
    }
}
