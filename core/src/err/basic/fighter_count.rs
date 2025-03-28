use crate::sol::Count;

#[derive(Debug)]
pub struct FighterCountError {
    pub count: Count,
}
impl std::error::Error for FighterCountError {}
impl std::fmt::Display for FighterCountError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fighter should be 1+, received {}", self.count)
    }
}
