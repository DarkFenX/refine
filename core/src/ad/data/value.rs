use crate::{
    def::{Count, OF, Value},
    ed::{EGenFloat, EGenInt},
};

#[derive(Copy, Clone)]
pub struct ACount(Count);
impl ACount {
    pub fn new(value: Count) -> Self {
        Self(value)
    }
    pub const fn into_inner(self) -> Count {
        self.0
    }
}
impl From<EGenInt> for ACount {
    fn from(value: EGenInt) -> Self {
        Self::new(value.into_inner().max(0) as Count)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AGenVal(Value);
impl AGenVal {
    pub fn new(value: Value) -> Self {
        Self(value)
    }
    pub fn new_f64(value: f64) -> Self {
        Self(OF(value))
    }
    pub const fn into_inner(self) -> Value {
        self.0
    }
}
impl From<EGenFloat> for AGenVal {
    fn from(value: EGenFloat) -> Self {
        Self::new(OF(value.into_inner()))
    }
}

#[derive(Copy, Clone)]
pub struct ATimeVal(Value);
impl ATimeVal {
    pub fn new_clamped(value: Value) -> Self {
        Self(value.max(OF(0.0)))
    }
    pub const fn into_inner(self) -> Value {
        self.0
    }
}
impl From<EGenFloat> for ATimeVal {
    fn from(value: EGenFloat) -> Self {
        Self::new_clamped(OF(value.into_inner()))
    }
}
