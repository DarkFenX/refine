use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::validation) struct HValRigSizeFail {
    allowed_size: f64,
    #[serde_as(as = "&Map<DisplayFromStr, _>")]
    rig_sizes: Vec<(rc::ItemId, Option<f64>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HValRigSizeFail {
    pub(in crate::info::validation) fn from_core(core_val_fail: rc::val::ValRigSizeFail) -> Self {
        Self {
            allowed_size: core_val_fail.allowed_size.into_f64(),
            rig_sizes: core_val_fail
                .rig_sizes
                .into_iter()
                .map(|(k, v)| (k, v.map(|v| v.into_f64())))
                .collect(),
        }
    }
}
