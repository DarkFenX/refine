use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(crate) struct HStatErps {
    shield: Option<HStatErpsLayerRegen>,
    armor: Option<HStatErpsLayer>,
    hull: Option<HStatErpsLayer>,
}
impl HStatErps {
    pub(crate) fn from_core(core_stat: rc::stats::StatErps) -> Self {
        Self {
            shield: core_stat.shield.map(HStatErpsLayerRegen::from_core),
            armor: core_stat.armor.map(HStatErpsLayer::from_core),
            hull: core_stat.hull.map(HStatErpsLayer::from_core),
        }
    }
}

#[derive(Serialize_tuple)]
struct HStatErpsLayerRegen {
    local: f64,
    remote: f64,
    remote_penalized: f64,
    regen: f64,
    mult: f64,
}
impl HStatErpsLayerRegen {
    fn from_core(core_stat: rc::stats::StatErpsLayerRegen) -> Self {
        Self {
            local: core_stat.local.into_f64(),
            remote: core_stat.remote.into_f64(),
            remote_penalized: core_stat.remote_penalized.into_f64(),
            regen: core_stat.regen.into_f64(),
            mult: core_stat.mult.into_f64(),
        }
    }
}

#[derive(Serialize_tuple)]
struct HStatErpsLayer {
    local: f64,
    remote: f64,
    remote_penalized: f64,
    mult: f64,
}
impl HStatErpsLayer {
    fn from_core(core_stat: rc::stats::StatErpsLayer) -> Self {
        Self {
            local: core_stat.local.into_f64(),
            remote: core_stat.remote.into_f64(),
            remote_penalized: core_stat.remote_penalized.into_f64(),
            mult: core_stat.mult.into_f64(),
        }
    }
}
