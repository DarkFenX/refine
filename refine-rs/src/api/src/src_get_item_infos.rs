use crate::{ItemTypeId, Src, src::SrcItemInfos};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src<'_> {
    /// Types which the source does not have are skipped, repeated type IDs are reported once.
    #[tracing::instrument(name = "src-itm-inf", level = "error", skip_all)]
    pub async fn get_item_infos(&self, type_ids: &[ItemTypeId]) -> SrcItemInfos {
        SrcItemInfos::from_core(self.inner.get_core(), type_ids)
    }
}
