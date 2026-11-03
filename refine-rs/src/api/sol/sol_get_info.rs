use crate::{Sol, SolInfo, SolInfoCmd};

impl Sol<'_> {
    #[tracing::instrument(name = "sol-inf", level = "error", skip_all)]
    pub async fn get_info(&mut self, info_cmd: SolInfoCmd) -> SolInfo {
        // Variables for move
        let ctx = self.get_ctx();
        self.exec_standard_infallible(move |core| info_cmd.execute(ctx.sol_id, ctx.src_alias, core))
            .await
    }
}
