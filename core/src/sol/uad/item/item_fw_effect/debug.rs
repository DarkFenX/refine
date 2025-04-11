use crate::sol::{
    debug::{DebugResult, check_fit_key},
    uad::Uad,
};

use super::UadFwEffect;

impl UadFwEffect {
    pub(in crate::sol::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        Ok(())
    }
}
