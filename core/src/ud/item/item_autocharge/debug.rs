use crate::{
    dbg::{DebugError, DebugResult},
    ud::{UAutocharge, UData},
};

impl UAutocharge {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        self.get_fit_uid().consistency_check(u_data)?;
        // Autocharges should exist only for containers which are loaded
        self.get_cont_item_uid().consistency_check(u_data, true)?;
        // Autocharges should exist only for effects available on current source
        self.get_cont_effect_rid().consistency_check(u_data)?;
        // Effect ID should be available on containing item
        if !u_data
            .items
            .get(self.get_cont_item_uid())
            .get_effects()
            .unwrap()
            .contains_key(&self.get_cont_effect_rid())
        {
            return Err(DebugError {});
        }
        self.get_projs().consistency_check(u_data)?;
        Ok(())
    }
}
