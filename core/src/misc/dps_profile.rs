use crate::{
    def::{AttrVal, OF},
    err::basic::DmgError,
    misc::BreacherInfo,
};

const DPS_MIN: AttrVal = OF(0.0);

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpsProfile {
    em: AttrVal,
    thermal: AttrVal,
    kinetic: AttrVal,
    explosive: AttrVal,
    breacher: Option<BreacherInfo>,
}
impl DpsProfile {
    pub fn new_clamped(
        em: AttrVal,
        thermal: AttrVal,
        kinetic: AttrVal,
        explosive: AttrVal,
        breacher: Option<BreacherInfo>,
    ) -> Self {
        Self {
            em: AttrVal::max(em, DPS_MIN),
            thermal: AttrVal::max(thermal, DPS_MIN),
            kinetic: AttrVal::max(kinetic, DPS_MIN),
            explosive: AttrVal::max(explosive, DPS_MIN),
            breacher,
        }
    }
    pub fn try_new(
        em: AttrVal,
        thermal: AttrVal,
        kinetic: AttrVal,
        explosive: AttrVal,
        breacher: Option<BreacherInfo>,
    ) -> Result<Self, DpsProfileError> {
        if em < DPS_MIN {
            return Err(DmgError::Em(em).into());
        }
        if thermal < DPS_MIN {
            return Err(DmgError::Thermal(em).into());
        }
        if kinetic < DPS_MIN {
            return Err(DmgError::Kinetic(em).into());
        }
        if explosive < DPS_MIN {
            return Err(DmgError::Explosive(em).into());
        }
        Ok(Self {
            em,
            thermal,
            kinetic,
            explosive,
            breacher,
        })
    }
    pub fn get_em(&self) -> AttrVal {
        self.em
    }
    pub fn get_thermal(&self) -> AttrVal {
        self.thermal
    }
    pub fn get_kinetic(&self) -> AttrVal {
        self.kinetic
    }
    pub fn get_explosive(&self) -> AttrVal {
        self.explosive
    }
    pub fn get_breacher(&self) -> Option<BreacherInfo> {
        self.breacher
    }
    pub(crate) fn deals_breacher_dps(&self) -> bool {
        match self.breacher {
            Some(breacher) => breacher.get_relative_max() > OF(0.0) && breacher.get_absolute_max() > OF(0.0),
            None => false,
        }
    }
    pub(crate) fn get_sum_regular(&self) -> AttrVal {
        self.em + self.thermal + self.kinetic + self.explosive
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DpsProfileError {
    #[error("{0}")]
    InvalidDmg(#[from] DmgError),
}
