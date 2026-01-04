use crate::ed::{EGenFloat, EGenInt};

#[derive(Copy, Clone)]
pub struct ACount(u32);
impl ACount {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
    pub fn into_inner(self) -> u32 {
        self.0
    }
}
impl From<EGenInt> for ACount {
    fn from(value: EGenInt) -> Self {
        Self::new(value.into_inner().max(0) as u32)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct AValue(f64);
impl AValue {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
    pub fn into_inner(self) -> f64 {
        self.0
    }
}
impl From<EGenFloat> for AValue {
    fn from(value: EGenFloat) -> Self {
        Self::new(value.into_inner())
    }
}
