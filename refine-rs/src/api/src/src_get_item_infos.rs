use crate::{
    Src,
    src::{SrcItemInfos, SrcItemInfosCmd},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src<'_> {
    /// Types which the source does not have are skipped, repeated type IDs are reported once.
    ///
    /// A command without type IDs gets all item types, like [`Self::iter_item_infos`].
    #[tracing::instrument(name = "src-itm-inf", level = "error", skip_all)]
    pub async fn get_item_infos(&self, info_cmd: SrcItemInfosCmd) -> SrcItemInfos {
        info_cmd.execute(self.inner.get_core())
    }
}
