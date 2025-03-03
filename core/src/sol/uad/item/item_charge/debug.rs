use crate::sol::{
    debug::{SolDebugResult, check_fit, check_item},
    uad::SolUad,
};

use super::SolCharge;

impl SolCharge {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        check_fit(uad, &self.get_fit_id())?;
        check_item(uad, &self.get_cont_id(), false)?;
        self.get_projs().debug_consistency_check(uad)?;
        Ok(())
    }
}
