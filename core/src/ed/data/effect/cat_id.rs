#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct EEffectCatId(i32);
impl EEffectCatId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}
