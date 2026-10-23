use super::main::{ItemMutationData, ItemMutationDataCache, UItemBaseMutable};
use crate::{dbg::DebugResult, ud::UData};

impl UItemBaseMutable {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self { base, mutation } = self;
        base.consistency_check(u_data)?;
        if let Some(mutation_data) = mutation {
            mutation_data.consistency_check(u_data)?;
        }
        Ok(())
    }
}

impl ItemMutationData {
    fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self {
            cache,
            // Not checked
            mutator_type_aid: _,
            attr_rolls: _,
        } = self;
        if let Some(mutation_cache) = cache {
            mutation_cache.consistency_check(u_data)?;
        }
        Ok(())
    }
}

impl ItemMutationDataCache {
    fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self {
            merged_attr_data,
            // Not checked
            base_type_aid: _,
            mutator: _,
        } = self;
        merged_attr_data.consistency_check(u_data)?;
        Ok(())
    }
}
