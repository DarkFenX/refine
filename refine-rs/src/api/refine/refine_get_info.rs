use crate::{
    Refine, RefineInfo, SolInfo, SolInfoCmd, SolInfoExt, SolInfoMode,
    src::{SrcInfo, SrcInfoMode},
};

impl Refine {
    #[tracing::instrument(name = "rfn-inf", level = "error", skip_all)]
    pub async fn get_info(&self, src_info_mode: SrcInfoMode, sol_info_cmd: SolInfoCmd) -> RefineInfo {
        RefineInfo {
            threads_standard: self.tpool.get_threads_standard(),
            threads_heavy: self.tpool.get_threads_heavy(),
            srcs: self.get_src_infos(src_info_mode),
            sols: self.get_sol_infos(sol_info_cmd).await,
        }
    }
    fn get_src_infos(&self, src_info_mode: SrcInfoMode) -> Vec<SrcInfo> {
        // Infos are built after releasing the lock, since full info clones warnings
        let inner_srcs: Vec<_> = {
            let alias_data = self.src_alias_data.read();
            alias_data
                .map
                .values()
                .map(|inner_src| (inner_src.clone(), alias_data.is_default(inner_src)))
                .collect()
        };
        inner_srcs
            .iter()
            .map(|(inner_src, is_default)| SrcInfo::from_inner(inner_src, *is_default, src_info_mode))
            .collect()
    }
    async fn get_sol_infos(&self, sol_info_cmd: SolInfoCmd) -> Vec<SolInfo> {
        let guarded_inner_sols = self.id_sol_map.get_all();
        match sol_info_cmd.get_sol_mode() {
            // ID and source alias are available without waiting for solar system locks
            SolInfoMode::Id => guarded_inner_sols
                .into_iter()
                .map(|(sol_id, _)| SolInfo::from_id_and_ext(sol_id, None))
                .collect(),
            SolInfoMode::Partial => guarded_inner_sols
                .into_iter()
                .map(|(sol_id, guarded_inner_sol)| {
                    let sol_info_ext = SolInfoExt::from_src_alias(guarded_inner_sol.get_src_alias());
                    SolInfo::from_id_and_ext(sol_id, Some(sol_info_ext))
                })
                .collect(),
            SolInfoMode::Full => {
                let mut sol_infos = Vec::with_capacity(guarded_inner_sols.len());
                for (_, guarded_inner_sol) in guarded_inner_sols {
                    // Solar systems are not touched, so that requesting refine info (which is not
                    // directly related to involved sols) does not keep them alive
                    if let Some(mut sol) = self.internal_lock_sol(guarded_inner_sol, false).await {
                        sol_infos.push(sol.get_info(sol_info_cmd.clone()).await);
                    }
                }
                sol_infos
            }
        }
    }
}
