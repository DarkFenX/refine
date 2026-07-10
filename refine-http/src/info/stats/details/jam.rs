use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(crate) struct HStatInJam {
    chance: f64,
    uptime: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatInJam {
    pub(crate) fn from_core(core_stat: rc::stats::StatInJam) -> Self {
        Self {
            chance: core_stat.chance.into_f64(),
            uptime: core_stat.uptime.into_f64(),
        }
    }
}
