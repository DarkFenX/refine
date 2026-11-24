use crate::{
    Src,
    src::{SrcInfoEnumCmd, SrcInfoEnumResp},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src<'_> {
    /// Responses are returned in the order of commands.
    #[tracing::instrument(name = "src-qry-btc", level = "error", skip_all)]
    pub async fn query_batch(&self, info_cmds: Vec<SrcInfoEnumCmd>) -> Vec<SrcInfoEnumResp> {
        let core_src = self.inner.get_core();
        info_cmds
            .into_iter()
            .map(|info_cmd| info_cmd.execute(core_src))
            .collect()
    }
}
