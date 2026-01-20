use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(crate) struct HStatRps {
    shield: HStatRpsLayerRegen,
    armor: HStatRpsLayer,
    hull: HStatRpsLayer,
}

#[derive(Serialize_tuple)]
struct HStatRpsLayerRegen {
    local: f64,
    remote: f64,
    remote_penalized: f64,
    regen: f64,
}

#[derive(Serialize_tuple)]
struct HStatRpsLayer {
    local: f64,
    remote: f64,
    remote_penalized: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatRps {
    pub(crate) fn from_core(core_stat: rc::stats::StatRps) -> Self {
        Self {
            shield: HStatRpsLayerRegen::from_core(core_stat.shield),
            armor: HStatRpsLayer::from_core(core_stat.armor),
            hull: HStatRpsLayer::from_core(core_stat.hull),
        }
    }
}

impl HStatRpsLayerRegen {
    fn from_core(core_stat: rc::stats::StatRpsLayerRegen) -> Self {
        Self {
            local: core_stat.local.into_f64(),
            remote: core_stat.remote.into_f64(),
            remote_penalized: core_stat.remote_penalized.into_f64(),
            regen: core_stat.regen.into_f64(),
        }
    }
}

impl HStatRpsLayer {
    fn from_core(core_stat: rc::stats::StatRpsLayer) -> Self {
        Self {
            local: core_stat.local.into_f64(),
            remote: core_stat.remote.into_f64(),
            remote_penalized: core_stat.remote_penalized.into_f64(),
        }
    }
}
