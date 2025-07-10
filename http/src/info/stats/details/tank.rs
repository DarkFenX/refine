#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatTank<T>
where
    T: serde::Serialize,
{
    shield: T,
    armor: T,
    hull: T,
}
impl<T, U> From<rc::stats::StatTank<U>> for HStatTank<T>
where
    T: serde::Serialize,
    U: Into<T>,
{
    fn from(core_stat: rc::stats::StatTank<U>) -> Self {
        Self {
            shield: core_stat.shield.into(),
            armor: core_stat.armor.into(),
            hull: core_stat.hull.into(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatLayerHp {
    buffer: rc::AttrVal,
    ancil_local: rc::AttrVal,
    ancil_remote: rc::AttrVal,
}
impl From<rc::stats::StatLayerHp> for HStatLayerHp {
    fn from(core_stat: rc::stats::StatLayerHp) -> Self {
        Self {
            buffer: core_stat.buffer,
            ancil_local: core_stat.ancil_local,
            ancil_remote: core_stat.ancil_remote,
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatLayerEhp {
    buffer: rc::AttrVal,
    ancil_local: rc::AttrVal,
    ancil_remote: rc::AttrVal,
    mult: rc::AttrVal,
}
impl From<rc::stats::StatLayerEhp> for HStatLayerEhp {
    fn from(core_stat: rc::stats::StatLayerEhp) -> Self {
        Self {
            buffer: core_stat.buffer,
            ancil_local: core_stat.ancil_local,
            ancil_remote: core_stat.ancil_remote,
            mult: core_stat.mult,
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatLayerResist {
    em: rc::AttrVal,
    thermal: rc::AttrVal,
    kinetic: rc::AttrVal,
    explosive: rc::AttrVal,
}
impl From<rc::stats::DmgKinds<rc::AttrVal>> for HStatLayerResist {
    fn from(core_stat: rc::stats::DmgKinds<rc::AttrVal>) -> Self {
        Self {
            em: core_stat.em,
            thermal: core_stat.thermal,
            kinetic: core_stat.kinetic,
            explosive: core_stat.explosive,
        }
    }
}
