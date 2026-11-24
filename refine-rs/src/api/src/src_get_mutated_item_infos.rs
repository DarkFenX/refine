use crate::{ItemTypeId, Src, src::SrcMutatedItemInfo, util::RSet};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src<'_> {
    /// Mutations are pairs of base type ID and mutator ID.
    ///
    /// Those which the source cannot apply are skipped, repeated ones are reported once.
    #[tracing::instrument(name = "src-mut-itm-inf", level = "error", skip_all)]
    pub async fn get_mutated_item_infos(&self, mutations: &[(ItemTypeId, ItemTypeId)]) -> Vec<SrcMutatedItemInfo> {
        let core_src = self.inner.get_core();
        let mut seen_mutations = RSet::default();
        mutations
            .iter()
            .filter(|&&mutation| seen_mutations.insert(mutation))
            .filter_map(|&(base_type_id, mutator_id)| {
                let core_src_item = core_src.get_mutated_item(base_type_id, mutator_id)?;
                Some(SrcMutatedItemInfo::from_core(base_type_id, mutator_id, core_src_item))
            })
            .collect()
    }
}
