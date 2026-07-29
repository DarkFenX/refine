use super::main::{ItemMutationData, ItemMutationDataCache, UItemBaseMutable};
use crate::{dbg::DebugResult, ud::UData};

impl UItemBaseMutable {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        if let Some(mutation_data) = self.get_mutation_data() {
            mutation_data.consistency_check(u_data)?;
        }
        Ok(())
    }
}

impl ItemMutationData {
    fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(mutation_cache) = self.get_cache() {
            mutation_cache.consistency_check(u_data)?;
        }
        Ok(())
    }
}

impl ItemMutationDataCache {
    fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.merged_attr_data.consistency_check(u_data)?;
        Ok(())
    }
}
