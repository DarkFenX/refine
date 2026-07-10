use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::validation) struct HValUnusableResFail {
    max: Option<f64>,
    #[serde_as(as = "&Map<DisplayFromStr, _>")]
    users: Vec<(rc::ItemId, f64)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HValUnusableResFail {
    pub(in crate::info::validation) fn from_core(core_val_fail: rc::val::ValUnusableResFail) -> Self {
        Self {
            max: core_val_fail.max.map(|v| v.into_f64()),
            users: core_val_fail
                .users
                .into_iter()
                .map(|(k, v)| (k, v.into_f64()))
                .collect(),
        }
    }
}
