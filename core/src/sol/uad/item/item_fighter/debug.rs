use crate::sol::{
    debug::{SolDebugResult, check_fit},
    uad::SolUad,
};

use super::SolFighter;

impl SolFighter {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        check_fit(uad, &self.get_fit_id())?;
        self.get_autocharges().debug_consistency_check(uad)?;
        self.get_projs().debug_consistency_check(uad)?;
        Ok(())
    }
}
