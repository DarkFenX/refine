use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(crate) struct HStatResource {
    used: f64,
    output: Option<f64>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatResource {
    pub(crate) fn from_core(core_stat: rc::stats::StatResource) -> Self {
        Self {
            used: core_stat.used.into_f64(),
            output: core_stat.output.map(|v| v.into_f64()),
        }
    }
}
