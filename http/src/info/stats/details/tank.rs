#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatTank<T>
where
    T: serde::Serialize,
{
    shield: T,
    armor: T,
    structure: T,
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
            structure: core_stat.structure.into(),
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
