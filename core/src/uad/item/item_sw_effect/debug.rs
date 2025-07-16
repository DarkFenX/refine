use crate::{
    dbg::{DebugResult, check_a_effect_id},
    uad::{Uad, UadSwEffect},
};

impl UadSwEffect {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        if let Some(reffs) = self.get_reffs() {
            for a_effect_id in reffs.iter() {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        Ok(())
    }
}
