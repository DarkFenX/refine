#[derive(Copy, Clone)]
pub struct EGenInt(i32);
impl EGenInt {
    pub fn new(value: i32) -> Self {
        Self(value)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone)]
pub struct EGenFloat(f64);
impl EGenFloat {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
    pub const fn into_inner(self) -> f64 {
        self.0
    }
}
