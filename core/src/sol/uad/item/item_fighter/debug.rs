use crate::sol::{
    debug::{DebugResult, check_fit_key},
    uad::Uad,
};

use super::Fighter;

impl Fighter {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        self.get_autocharges().debug_consistency_check(uad)?;
        self.get_projs().debug_consistency_check(uad)?;
        Ok(())
    }
}
