use crate::{
    Sol,
    stats::{SolStatsCmd, SolStatsResp},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol<'_> {
    #[tracing::instrument(name = "sol-stt", level = "error", skip_all)]
    pub async fn get_stats(&mut self, stats_cmd: SolStatsCmd) -> SolStatsResp {
        self.exec_standard_infallible(|core_sol| stats_cmd.execute(core_sol))
            .await
    }
}
