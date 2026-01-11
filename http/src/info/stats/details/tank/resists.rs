use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(crate) struct HStatResists {
    shield: HStatResistsLayer,
    armor: HStatResistsLayer,
    hull: HStatResistsLayer,
}
impl HStatResists {
    pub(crate) fn from_core(core_stat: rc::stats::StatResists) -> Self {
        Self {
            shield: HStatResistsLayer::from_core(core_stat.shield),
            armor: HStatResistsLayer::from_core(core_stat.armor),
            hull: HStatResistsLayer::from_core(core_stat.hull),
        }
    }
}

#[derive(Serialize_tuple)]
struct HStatResistsLayer {
    em: f64,
    thermal: f64,
    kinetic: f64,
    explosive: f64,
}
impl HStatResistsLayer {
    fn from_core(core_stat: rc::stats::StatResistsLayer) -> Self {
        Self {
            em: core_stat.em.into_f64(),
            thermal: core_stat.thermal.into_f64(),
            kinetic: core_stat.kinetic.into_f64(),
            explosive: core_stat.explosive.into_f64(),
        }
    }
}
