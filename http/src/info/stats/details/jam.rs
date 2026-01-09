use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(crate) struct HStatJamApplied {
    chance: f64,
    uptime: f64,
}
impl From<rc::stats::StatJamApplied> for HStatJamApplied {
    fn from(core_stat: rc::stats::StatJamApplied) -> Self {
        Self {
            chance: core_stat.chance.into_f64(),
            uptime: core_stat.uptime.into_f64(),
        }
    }
}
