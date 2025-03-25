use crate::sol::{
    debug::{DebugResult, check_fit_id},
    uad::Uad,
};

use super::Rig;

impl Rig {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_id(uad, &self.get_fit_id())?;
        Ok(())
    }
}
