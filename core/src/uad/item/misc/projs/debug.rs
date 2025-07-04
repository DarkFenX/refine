use crate::{
    dbg::{DebugResult, check_item_key},
    uad::{Uad, item::misc::Projs},
};

impl Projs {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for (projectee_item_key, proj_range) in self.data.iter() {
            // Projectee is not necessarily loaded
            check_item_key(uad, *projectee_item_key, false)?;
            if let Some(proj_range) = proj_range {
                proj_range.consistency_check()?;
            }
        }
        Ok(())
    }
}
