use super::UadFighter;
use crate::sol::{
    debug::{DebugResult, check_fit_key},
    uad::Uad,
};

impl UadFighter {
    pub(in crate::sol::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        self.get_autocharges().consistency_check(uad)?;
        self.get_projs().consistency_check(uad)?;
        Ok(())
    }
}
