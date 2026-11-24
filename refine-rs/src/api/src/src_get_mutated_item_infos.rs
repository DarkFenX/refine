use crate::{ItemTypeId, Src, src::SrcMutatedItemInfos};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src<'_> {
    /// Mutations are pairs of base type ID and mutator ID.
    ///
    /// Those which the source cannot apply are skipped, repeated ones are reported once.
    #[tracing::instrument(name = "src-mut-itm-inf", level = "error", skip_all)]
    pub async fn get_mutated_item_infos(&self, mutations: &[(ItemTypeId, ItemTypeId)]) -> SrcMutatedItemInfos {
        SrcMutatedItemInfos::from_core(self.inner.get_core(), mutations)
    }
}
