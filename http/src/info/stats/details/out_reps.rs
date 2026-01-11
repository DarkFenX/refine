use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(crate) struct HStatOutReps {
    shield: f64,
    armor: f64,
    hull: f64,
}
impl HStatOutReps {
    pub(crate) fn from_core(core_stat: rc::stats::StatOutReps) -> Self {
        Self {
            shield: core_stat.shield.into_f64(),
            armor: core_stat.armor.into_f64(),
            hull: core_stat.hull.into_f64(),
        }
    }
}
