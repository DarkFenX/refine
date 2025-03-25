use crate::sol::{
    debug::{DebugResult, check_fit_id},
    uad::Uad,
};

use super::Stance;

impl Stance {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_id(uad, &self.get_fit_id())?;
        Ok(())
    }
}
