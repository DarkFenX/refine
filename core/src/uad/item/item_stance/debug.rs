use crate::{
    dbg::{DebugResult, check_a_effect_id, check_fit_key},
    uad::{Uad, UadStance},
};

impl UadStance {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        if let Some(reffs) = self.get_reffs() {
            for a_effect_id in reffs.iter() {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        check_fit_key(uad, self.get_fit_key())?;
        Ok(())
    }
}
