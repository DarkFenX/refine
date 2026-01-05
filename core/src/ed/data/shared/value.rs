#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct EGenInt(i32);
impl EGenInt {
    pub fn new(value: i32) -> Self {
        Self(value)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone, PartialEq, derive_more::Display)]
pub struct EGenFloat(f64);
impl EGenFloat {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
    pub const fn into_inner(self) -> f64 {
        self.0
    }
}
