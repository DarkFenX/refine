#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatTank<T>
where
    T: serde::Serialize,
{
    shield: T,
    armor: T,
    hull: T,
}
impl<T> HStatTank<Option<T>>
where
    T: serde::Serialize,
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
    T: serde::Serialize,
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

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatTankRegen<T, U>
where
    T: serde::Serialize,
    U: serde::Serialize,
{
    shield: U,
    armor: T,
    hull: T,
}
impl<T, U> HStatTankRegen<Option<T>, Option<U>>
where
    T: serde::Serialize,
    U: serde::Serialize,
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
    T: serde::Serialize,
    U: serde::Serialize,
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
pub(crate) struct HStatLayerRps {
    local: rc::AttrVal,
    remote: rc::AttrVal,
    remote_penalized: rc::AttrVal,
}
impl From<rc::stats::StatLayerRps> for HStatLayerRps {
    fn from(core_stat: rc::stats::StatLayerRps) -> Self {
        Self {
            local: core_stat.local,
            remote: core_stat.remote,
            remote_penalized: core_stat.remote_penalized,
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatLayerRpsRegen {
    local: rc::AttrVal,
    remote: rc::AttrVal,
    remote_penalized: rc::AttrVal,
    regen: rc::AttrVal,
}
impl From<rc::stats::StatLayerRpsRegen> for HStatLayerRpsRegen {
    fn from(core_stat: rc::stats::StatLayerRpsRegen) -> Self {
        Self {
            local: core_stat.local,
            remote: core_stat.remote,
            remote_penalized: core_stat.remote_penalized,
            regen: core_stat.regen,
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatLayerErps {
    local: rc::AttrVal,
    remote: rc::AttrVal,
    remote_penalized: rc::AttrVal,
    mult: rc::AttrVal,
}
impl From<rc::stats::StatLayerErps> for HStatLayerErps {
    fn from(core_stat: rc::stats::StatLayerErps) -> Self {
        Self {
            local: core_stat.local,
            remote: core_stat.remote,
            remote_penalized: core_stat.remote_penalized,
            mult: core_stat.mult,
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatLayerErpsRegen {
    local: rc::AttrVal,
    remote: rc::AttrVal,
    remote_penalized: rc::AttrVal,
    regen: rc::AttrVal,
    mult: rc::AttrVal,
}
impl From<rc::stats::StatLayerErpsRegen> for HStatLayerErpsRegen {
    fn from(core_stat: rc::stats::StatLayerErpsRegen) -> Self {
        Self {
            local: core_stat.local,
            remote: core_stat.remote,
            remote_penalized: core_stat.remote_penalized,
            regen: core_stat.regen,
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
