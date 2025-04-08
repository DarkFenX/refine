use crate::sol::{
    debug::{DebugResult, check_fit_key},
    uad::Uad,
};

use super::Skill;

impl Skill {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        Ok(())
    }
}
