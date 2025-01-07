use crate::sol::{
    uad::{item::debug, SolUad},
    SolDebugResult,
};

use super::SolAutocharge;

impl SolAutocharge {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        debug::check_fit(uad, &self.get_fit_id())?;
        debug::check_item(uad, &self.get_cont_id())?;
        self.get_projs().debug_consistency_check(uad)?;
        Ok(())
    }
}
