use crate::{
    dbg::{DebugResult, check_attr_rid, check_effect_rid},
    ud::{UData, item::base::UItemBaseMutable},
};

impl UItemBaseMutable {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        if let Some(mutation_data) = self.get_mutation_data()
            && let Some(mutation_cache) = mutation_data.get_cache()
        {
            for &attr_rid in mutation_cache.merged_attrs.keys() {
                check_attr_rid(u_data, attr_rid)?;
            }
            if let Some(effect_data) = &mutation_cache.merged_effdatas {
                for &effect_rid in effect_data.keys() {
                    check_effect_rid(u_data, effect_rid)?;
                }
            }
            mutation_cache.axt.consistency_check(u_data)?;
        }
        Ok(())
    }
}
