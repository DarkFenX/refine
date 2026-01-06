#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct EInt(i32);
impl EInt {
    pub const fn from_i32(value: i32) -> Self {
        Self(value)
    }
    pub const fn into_i32(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone, PartialEq, derive_more::Display)]
pub struct EFloat(f64);
impl EFloat {
    pub const fn from_f64(value: f64) -> Self {
        Self(value)
    }
    pub const fn into_f64(self) -> f64 {
        self.0
    }
}
