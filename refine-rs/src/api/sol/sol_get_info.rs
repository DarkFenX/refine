use crate::{SolInfo, SolInfoCmd, SolarSystem};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-inf", level = "trace", skip_all)]
    pub async fn get_info(&mut self, info_cmd: SolInfoCmd) -> SolInfo {
        self.exec_standard_safe_ctx(|ctx, core_sol| info_cmd.execute(ctx.sol_id, ctx.src_alias, core_sol))
            .await
    }
}
