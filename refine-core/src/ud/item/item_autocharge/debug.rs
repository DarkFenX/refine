use crate::{
    dbg::DebugResult,
    ud::{UAutocharge, UData},
};

impl UAutocharge {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self {
            base,
            fit_uid,
            cont_item_uid,
            cont_effect_rid,
            projs,
            // Not checked
            activated: _,
            force_disabled: _,
        } = self;
        base.consistency_check(u_data)?;
        fit_uid.consistency_check(u_data)?;
        // Autocharges should exist only for containers which are loaded
        cont_item_uid.consistency_check(u_data, true)?;
        // Autocharges should exist only for effects available on current source
        cont_effect_rid.consistency_check(u_data)?;
        // Effect ID should be available on containing item
        if !u_data
            .items
            .get(*cont_item_uid)
            .get_r_item_base()
            .unwrap()
            .effects
            .contains_key(cont_effect_rid)
        {
            return Err(Default::default());
        }
        projs.consistency_check(u_data)?;
        Ok(())
    }
}
