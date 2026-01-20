use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::validation) struct HValCapitalModFail {
    max_subcap_volume: f64,
    #[serde_as(as = "&Map<DisplayFromStr, _>")]
    module_volumes: Vec<(rc::ItemId, f64)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HValCapitalModFail {
    pub(in crate::info::validation) fn from_core(core_val_fail: rc::val::ValCapitalModFail) -> Self {
        Self {
            max_subcap_volume: core_val_fail.max_subcap_volume.into_f64(),
            module_volumes: core_val_fail
                .module_volumes
                .into_iter()
                .map(|(k, v)| (k, v.into_f64()))
                .collect(),
        }
    }
}
