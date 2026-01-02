use crate::{
    dbg::{DebugError, DebugResult, check_effect_id, check_fit_id, check_item_id},
    ud::{UAutocharge, UData},
};

impl UAutocharge {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        check_fit_id(u_data, self.get_fit_key())?;
        // All autocharges are supposed to be loaded
        check_item_id(u_data, self.get_cont_item_key(), true)?;
        // Autocharges should exist only for effects available on current source
        check_effect_id(u_data, self.get_cont_effect_key())?;
        // Effect key should be available on containing item
        if !u_data
            .items
            .get(self.get_cont_item_key())
            .get_effect_datas()
            .unwrap()
            .contains_key(&self.get_cont_effect_key())
        {
            return Err(DebugError {});
        }
        self.get_projs().consistency_check(u_data)?;
        Ok(())
    }
}
