use crate::sol::{
    debug::{DebugResult, check_fit_id},
    uad::Uad,
};

use super::Fighter;

impl Fighter {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_id(uad, &self.get_fit_id())?;
        self.get_autocharges().debug_consistency_check(uad)?;
        self.get_projs().debug_consistency_check(uad)?;
        Ok(())
    }
}
