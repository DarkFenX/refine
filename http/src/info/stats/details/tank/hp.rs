use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(crate) struct HStatHp {
    shield: HStatHpLayer,
    armor: HStatHpLayer,
    hull: HStatHpLayer,
}

#[derive(Serialize_tuple)]
struct HStatHpLayer {
    buffer: f64,
    ancil_local: f64,
    ancil_remote: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatHp {
    pub(crate) fn from_core(core_stat: rc::stats::StatHp) -> Self {
        Self {
            shield: HStatHpLayer::from_core(core_stat.shield),
            armor: HStatHpLayer::from_core(core_stat.armor),
            hull: HStatHpLayer::from_core(core_stat.hull),
        }
    }
}

impl HStatHpLayer {
    fn from_core(core_stat: rc::stats::StatHpLayer) -> Self {
        Self {
            buffer: core_stat.buffer.into_f64(),
            ancil_local: core_stat.ancil_local.into_f64(),
            ancil_remote: core_stat.ancil_remote.into_f64(),
        }
    }
}
