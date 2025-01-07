use crate::sol::{
    uad::{item::debug, SolUad},
    SolDebugResult,
};

use super::SolBooster;

impl SolBooster {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        debug::check_fit(uad, &self.get_fit_id())?;
        Ok(())
    }
}
