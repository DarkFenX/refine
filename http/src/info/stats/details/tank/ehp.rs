use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(crate) struct HStatEhp {
    shield: Option<HStatEhpLayer>,
    armor: Option<HStatEhpLayer>,
    hull: Option<HStatEhpLayer>,
}
impl HStatEhp {
    pub(crate) fn from_core(core_stat: rc::stats::StatEhp) -> Self {
        Self {
            shield: core_stat.shield.map(HStatEhpLayer::from_core),
            armor: core_stat.armor.map(HStatEhpLayer::from_core),
            hull: core_stat.hull.map(HStatEhpLayer::from_core),
        }
    }
}

#[derive(Serialize_tuple)]
struct HStatEhpLayer {
    buffer: f64,
    ancil_local: f64,
    ancil_remote: f64,
    mult: f64,
}
impl HStatEhpLayer {
    fn from_core(core_stat: rc::stats::StatEhpLayer) -> Self {
        Self {
            buffer: core_stat.buffer.into_f64(),
            ancil_local: core_stat.ancil_local.into_f64(),
            ancil_remote: core_stat.ancil_remote.into_f64(),
            mult: core_stat.mult.into_f64(),
        }
    }
}
