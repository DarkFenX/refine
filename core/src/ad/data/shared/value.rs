use crate::ed::{EFloat, EInt};

#[derive(Copy, Clone)]
pub struct ACount(u32);
impl ACount {
    pub fn from_u32(value: u32) -> Self {
        Self(value)
    }
    pub fn into_u32(self) -> u32 {
        self.0
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct AValue(f64);
impl AValue {
    pub fn from_f64(value: f64) -> Self {
        Self(value)
    }
    pub fn into_f64(self) -> f64 {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ACount {
    pub(in crate::ad) fn from_eint_clamped(eint: EInt) -> Self {
        Self(eint.into_i32().max(0) as u32)
    }
}
impl AValue {
    pub(in crate::ad) fn from_efloat(efloat: EFloat) -> Self {
        Self(efloat.into_f64())
    }
}
