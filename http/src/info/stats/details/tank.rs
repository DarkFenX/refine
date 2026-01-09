use serde::Serialize;
use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(crate) struct HStatTank<T>
where
    T: Serialize,
{
    shield: T,
    armor: T,
    hull: T,
}
impl<T> HStatTank<Option<T>>
where
    T: Serialize,
{
    pub(crate) fn from_opt<U>(core_stat: rc::stats::StatTank<Option<U>>) -> Self
    where
        U: Into<T>,
    {
        Self {
            shield: core_stat.shield.map(Into::into),
            armor: core_stat.armor.map(Into::into),
            hull: core_stat.hull.map(Into::into),
        }
    }
}
impl<T, CT> From<rc::stats::StatTank<CT>> for HStatTank<T>
where
    T: Serialize,
    CT: Into<T>,
{
    fn from(core_stat: rc::stats::StatTank<CT>) -> Self {
        Self {
            shield: core_stat.shield.into(),
            armor: core_stat.armor.into(),
            hull: core_stat.hull.into(),
        }
    }
}

#[derive(Serialize_tuple)]
pub(crate) struct HStatTankRegen<T, U>
where
    T: Serialize,
    U: Serialize,
{
    shield: U,
    armor: T,
    hull: T,
}
impl<T, U> HStatTankRegen<Option<T>, Option<U>>
where
    T: Serialize,
    U: Serialize,
{
    pub(crate) fn from_opt<CT, CU>(core_stat: rc::stats::StatTankRegen<Option<CT>, Option<CU>>) -> Self
    where
        CT: Into<T>,
        CU: Into<U>,
    {
        Self {
            shield: core_stat.shield.map(Into::into),
            armor: core_stat.armor.map(Into::into),
            hull: core_stat.hull.map(Into::into),
        }
    }
}
impl<T, U, CT, CU> From<rc::stats::StatTankRegen<CT, CU>> for HStatTankRegen<T, U>
where
    T: Serialize,
    U: Serialize,
    CT: Into<T>,
    CU: Into<U>,
{
    fn from(core_stat: rc::stats::StatTankRegen<CT, CU>) -> Self {
        Self {
            shield: core_stat.shield.into(),
            armor: core_stat.armor.into(),
            hull: core_stat.hull.into(),
        }
    }
}

#[derive(Serialize_tuple)]
pub(crate) struct HStatLayerHp {
    buffer: f64,
    ancil_local: f64,
    ancil_remote: f64,
}
impl From<rc::stats::StatLayerHp> for HStatLayerHp {
    fn from(core_stat: rc::stats::StatLayerHp) -> Self {
        Self {
            buffer: core_stat.buffer.into_f64(),
            ancil_local: core_stat.ancil_local.into_f64(),
            ancil_remote: core_stat.ancil_remote.into_f64(),
        }
    }
}

#[derive(Serialize_tuple)]
pub(crate) struct HStatLayerEhp {
    buffer: f64,
    ancil_local: f64,
    ancil_remote: f64,
    mult: f64,
}
impl From<rc::stats::StatLayerEhp> for HStatLayerEhp {
    fn from(core_stat: rc::stats::StatLayerEhp) -> Self {
        Self {
            buffer: core_stat.buffer.into_f64(),
            ancil_local: core_stat.ancil_local.into_f64(),
            ancil_remote: core_stat.ancil_remote.into_f64(),
            mult: core_stat.mult.into_f64(),
        }
    }
}

#[derive(Serialize_tuple)]
pub(crate) struct HStatLayerRps {
    local: f64,
    remote: f64,
    remote_penalized: f64,
}
impl From<rc::stats::StatLayerRps> for HStatLayerRps {
    fn from(core_stat: rc::stats::StatLayerRps) -> Self {
        Self {
            local: core_stat.local.into_f64(),
            remote: core_stat.remote.into_f64(),
            remote_penalized: core_stat.remote_penalized.into_f64(),
        }
    }
}

#[derive(Serialize_tuple)]
pub(crate) struct HStatLayerRpsRegen {
    local: f64,
    remote: f64,
    remote_penalized: f64,
    regen: f64,
}
impl From<rc::stats::StatLayerRpsRegen> for HStatLayerRpsRegen {
    fn from(core_stat: rc::stats::StatLayerRpsRegen) -> Self {
        Self {
            local: core_stat.local.into_f64(),
            remote: core_stat.remote.into_f64(),
            remote_penalized: core_stat.remote_penalized.into_f64(),
            regen: core_stat.regen.into_f64(),
        }
    }
}

#[derive(Serialize_tuple)]
pub(crate) struct HStatLayerErps {
    local: f64,
    remote: f64,
    remote_penalized: f64,
    mult: f64,
}
impl From<rc::stats::StatLayerErps> for HStatLayerErps {
    fn from(core_stat: rc::stats::StatLayerErps) -> Self {
        Self {
            local: core_stat.local.into_f64(),
            remote: core_stat.remote.into_f64(),
            remote_penalized: core_stat.remote_penalized.into_f64(),
            mult: core_stat.mult.into_f64(),
        }
    }
}

#[derive(Serialize_tuple)]
pub(crate) struct HStatLayerErpsRegen {
    local: f64,
    remote: f64,
    remote_penalized: f64,
    regen: f64,
    mult: f64,
}
impl From<rc::stats::StatLayerErpsRegen> for HStatLayerErpsRegen {
    fn from(core_stat: rc::stats::StatLayerErpsRegen) -> Self {
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
pub(crate) struct HStatLayerResist {
    em: f64,
    thermal: f64,
    kinetic: f64,
    explosive: f64,
}
impl From<rc::stats::DmgKinds<rc::UnitInterval>> for HStatLayerResist {
    fn from(core_stat: rc::stats::DmgKinds<rc::UnitInterval>) -> Self {
        Self {
            em: core_stat.em.into_f64(),
            thermal: core_stat.thermal.into_f64(),
            kinetic: core_stat.kinetic.into_f64(),
            explosive: core_stat.explosive.into_f64(),
        }
    }
}
