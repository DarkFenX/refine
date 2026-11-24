use crate::{
    Src,
    src::{SrcMutatedItemInfos, SrcMutatedItemInfosCmd},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src<'_> {
    /// Mutations which the source cannot apply are skipped, repeated ones are reported once.
    #[tracing::instrument(name = "src-mut-itm-inf", level = "error", skip_all)]
    pub async fn get_mutated_item_infos(&self, info_cmd: SrcMutatedItemInfosCmd) -> SrcMutatedItemInfos {
        info_cmd.execute(self.inner.get_core())
    }
}
