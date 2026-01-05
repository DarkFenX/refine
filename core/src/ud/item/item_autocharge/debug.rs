use crate::{
    dbg::{DebugError, DebugResult, check_effect_rid, check_fit_uid, check_item_uid},
    ud::{UAutocharge, UData},
};

impl UAutocharge {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        check_fit_uid(u_data, self.get_fit_uid())?;
        // All autocharges are supposed to be loaded
        check_item_uid(u_data, self.get_cont_item_uid(), true)?;
        // Autocharges should exist only for effects available on current source
        check_effect_rid(u_data, self.get_cont_effect_rid())?;
        // Effect ID should be available on containing item
        if !u_data
            .items
            .get(self.get_cont_item_uid())
            .get_effect_datas()
            .unwrap()
            .contains_key(&self.get_cont_effect_rid())
        {
            return Err(DebugError {});
        }
        self.get_projs().consistency_check(u_data)?;
        Ok(())
    }
}
