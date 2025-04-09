use crate::sol::Count;

#[derive(thiserror::Error, Debug)]
#[error("fighter should be 1+, received {count}")]
pub struct FighterCountError {
    pub count: Count,
}
