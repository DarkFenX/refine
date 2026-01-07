use crate::{
    err::basic::DmgError,
    misc::{Breacher, PValue, UnitInterval, Value},
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
    pub fn new(em: PValue, thermal: PValue, kinetic: PValue, explosive: PValue, breacher: Option<Breacher>) -> Self {
        Self {
            em,
            thermal,
            kinetic,
            explosive,
            breacher,
        }
    }
    pub fn new_clamped(
        em: Value,
        thermal: Value,
        kinetic: Value,
        explosive: Value,
        breacher: Option<Breacher>,
    ) -> Self {
        Self::new(
            PValue::from_value_clamped(em),
            PValue::from_value_clamped(thermal),
            PValue::from_value_clamped(kinetic),
            PValue::from_value_clamped(explosive),
            breacher,
        )
    }
    pub fn try_new(
        em: Value,
        thermal: Value,
        kinetic: Value,
        explosive: Value,
        breacher: Option<Breacher>,
    ) -> Result<Self, DpsProfileError> {
        let em = match em >= Value::ZERO {
            true => PValue::from_val_unchecked(em),
            false => return Err(DmgError::Em(em).into()),
        };
        let thermal = match thermal >= Value::ZERO {
            true => PValue::from_val_unchecked(thermal),
            false => return Err(DmgError::Thermal(thermal).into()),
        };
        let kinetic = match kinetic >= Value::ZERO {
            true => PValue::from_val_unchecked(kinetic),
            false => return Err(DmgError::Kinetic(kinetic).into()),
        };
        let explosive = match explosive >= Value::ZERO {
            true => PValue::from_val_unchecked(explosive),
            false => return Err(DmgError::Explosive(explosive).into()),
        };
        Ok(Self::new(em, thermal, kinetic, explosive, breacher))
    }
    pub fn get_em(&self) -> PValue {
        self.em
    }
    pub fn get_thermal(&self) -> PValue {
        self.thermal
    }
    pub fn get_kinetic(&self) -> PValue {
        self.kinetic
    }
    pub fn get_explosive(&self) -> PValue {
        self.explosive
    }
    pub fn get_breacher(&self) -> Option<Breacher> {
        self.breacher
    }
    pub(crate) fn deals_breacher_dps(&self) -> bool {
        match self.breacher {
            Some(breacher) => {
                breacher.get_relative_max() > UnitInterval::ZERO && breacher.get_absolute_max() > PValue::ZERO
            }
            None => false,
        }
    }
    pub(crate) fn get_sum_regular(&self) -> PValue {
        self.em + self.thermal + self.kinetic + self.explosive
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DpsProfileError {
    #[error("{0}")]
    InvalidDmg(#[from] DmgError),
}
