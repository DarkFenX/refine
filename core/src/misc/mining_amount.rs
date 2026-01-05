use crate::misc::Value;

#[derive(Copy, Clone)]
pub struct MiningAmount {
    pub yield_: Value,
    pub drain: Value,
}
impl Default for MiningAmount {
    fn default() -> Self {
        Self {
            yield_: Value::default(),
            drain: Value::default(),
        }
    }
}
