use std::num::Wrapping;

use crate::util::{LibDefault, LibIncrement};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct FitId(u32);
impl FitId {
    pub fn from_u32(id: u32) -> Self {
        Self(id)
    }
    pub fn into_u32(self) -> u32 {
        self.0
    }
}
impl LibDefault for FitId {
    fn lib_default() -> Self {
        Self(0)
    }
}
impl LibIncrement for FitId {
    fn lib_increment(&mut self) {
        self.0 = (Wrapping(self.0) + Wrapping(1)).0;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Error
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(thiserror::Error, Debug)]
#[error("fit {fit_id} not found")]
pub struct FitFoundError {
    pub fit_id: FitId,
}
// Conversion needed for unified user entity container to work
impl From<FitId> for FitFoundError {
    fn from(fit_id: FitId) -> Self {
        Self { fit_id }
    }
}
