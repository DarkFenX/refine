use crate::{
    err::basic::DmgError,
    misc::{Breacher, PValue},
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpsProfile {
    em: PValue,
    thermal: PValue,
    kinetic: PValue,
    explosive: PValue,
    breacher: Option<Breacher>,
}
impl DpsProfile {
    pub fn new_clamped(em: f64, thermal: f64, kinetic: f64, explosive: f64, breacher: Option<Breacher>) -> Self {
        Self {
            em: PValue::from_f64_clamped(em),
            thermal: PValue::from_f64_clamped(thermal),
            kinetic: PValue::from_f64_clamped(kinetic),
            explosive: PValue::from_f64_clamped(explosive),
            breacher,
        }
    }
    pub fn try_new(
        em: f64,
        thermal: f64,
        kinetic: f64,
        explosive: f64,
        breacher: Option<Breacher>,
    ) -> Result<Self, DpsProfileError> {
        if em < 0.0 {
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
    pub fn get_breacher(&self) -> Option<Breacher> {
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
