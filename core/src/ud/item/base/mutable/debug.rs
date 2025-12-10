use crate::{
    dbg::{DebugResult, check_attr_key, check_effect_key},
    ud::{UData, item::base::UItemBaseMutable},
};

impl UItemBaseMutable {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        if let Some(mutation_data) = self.get_mutation_data()
            && let Some(mutation_cache) = mutation_data.get_cache()
        {
            for &attr_key in mutation_cache.merged_attrs.keys() {
                check_attr_key(u_data, attr_key)?;
            }
            if let Some(effect_data) = &mutation_cache.merged_effdatas {
                for &effect_key in effect_data.keys() {
                    check_effect_key(u_data, effect_key)?;
                }
            }
            mutation_cache.axt.consistency_check(u_data)?;
        }
        Ok(())
    }
}
