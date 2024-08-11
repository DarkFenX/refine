use crate::defs::EEffectId;

#[derive(Debug)]
pub struct SideEffectError {
    pub effect_id: EEffectId,
}
impl SideEffectError {
    pub(crate) fn new(effect_id: EEffectId) -> Self {
        Self { effect_id }
    }
}
impl std::error::Error for SideEffectError {}
impl std::fmt::Display for SideEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "effect {} is not a side effect", self.effect_id)
    }
}
