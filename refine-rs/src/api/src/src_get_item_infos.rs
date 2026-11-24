use crate::{ItemTypeId, Src, src::SrcItemInfo, util::RSet};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src<'_> {
    /// Types which the source does not have are skipped, repeated type IDs are reported once.
    #[tracing::instrument(name = "src-itm-inf", level = "error", skip_all)]
    pub async fn get_item_infos(&self, type_ids: &[ItemTypeId]) -> Vec<SrcItemInfo> {
        let core_src = self.inner.get_core();
        let mut seen_type_ids = RSet::default();
        type_ids
            .iter()
            .filter(|&&type_id| seen_type_ids.insert(type_id))
            .filter_map(|&type_id| core_src.get_item(type_id).map(SrcItemInfo::from_core))
            .collect()
    }
}
