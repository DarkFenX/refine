use crate::util::LibDefault;

const COUNT_MIN: u32 = 1;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct FighterCount(u32);
impl FighterCount {
    pub fn new_checked(count: u32) -> Result<Self, FighterCountError> {
        match (COUNT_MIN..).contains(&count) {
            true => Ok(Self(count)),
            false => Err(FighterCountError { count }),
        }
    }
    pub fn new_clamped(count: u32) -> Self {
        Self(count.max(COUNT_MIN))
    }
    pub(crate) fn new_f64_clamped(count: f64) -> Self {
        Self(count.clamp(1.0, u32::MAX as f64).round() as u32)
    }
    pub fn into_inner(self) -> u32 {
        self.0
    }
}
impl LibDefault for FighterCount {
    fn lib_default() -> Self {
        Self(COUNT_MIN)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("fighter count should be 1+, received {count}")]
pub struct FighterCountError {
    pub count: u32,
}
