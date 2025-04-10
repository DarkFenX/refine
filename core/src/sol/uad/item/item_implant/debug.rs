use crate::sol::{
    debug::{DebugResult, check_fit_key},
    uad::Uad,
};

use super::Implant;

impl Implant {
    pub(in crate::sol::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        Ok(())
    }
}
