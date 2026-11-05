use crate::{
    FitInfoMode, FleetInfoMode, ItemInfoMode, Refine, RefineInfo, SolInfo, SolInfoCmd, SolInfoExt, SolInfoMode,
    src::{SrcInfo, SrcInfoMode},
};

/// Refine instance info request command.
///
/// Fleet, fit and item info modes apply to all solar systems. There are no per-ID overrides, since
/// those IDs are unique only within a solar system.
#[derive(Copy, Clone, Default)]
pub struct RefineInfoCmd {
    src_mode: SrcInfoMode,
    sol_mode: SolInfoMode,
    fleet_mode: FleetInfoMode,
    fit_mode: FitInfoMode,
    item_mode: ItemInfoMode,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RefineInfoCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_src(mut self, mode: SrcInfoMode) -> Self {
        self.src_mode = mode;
        self
    }
    pub fn with_sol(mut self, mode: SolInfoMode) -> Self {
        self.sol_mode = mode;
        self
    }
    pub fn with_fleet(mut self, mode: FleetInfoMode) -> Self {
        self.fleet_mode = mode;
        self
    }
    pub fn with_fit(mut self, mode: FitInfoMode) -> Self {
        self.fit_mode = mode;
        self
    }
    pub fn with_item(mut self, mode: ItemInfoMode) -> Self {
        self.item_mode = mode;
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RefineInfoCmd {
    pub(crate) async fn execute(self, refine: &Refine) -> RefineInfo {
        RefineInfo {
            threads_standard: refine.tpool.get_threads_standard(),
            threads_heavy: refine.tpool.get_threads_heavy(),
            srcs: self.get_src_infos(refine),
            sols: self.get_sol_infos(refine).await,
        }
    }
    fn get_src_infos(&self, refine: &Refine) -> Vec<SrcInfo> {
        // Infos are built after releasing the lock, since full info clones warnings
        let inner_srcs: Vec<_> = {
            let alias_data = refine.src_alias_data.read();
            alias_data
                .map
                .values()
                .map(|inner_src| (inner_src.clone(), alias_data.is_default(inner_src)))
                .collect()
        };
        inner_srcs
            .iter()
            .map(|(inner_src, is_default)| SrcInfo::from_inner(inner_src, *is_default, self.src_mode))
            .collect()
    }
    async fn get_sol_infos(&self, refine: &Refine) -> Vec<SolInfo> {
        let guarded_inner_sols = refine.id_sol_map.get_all();
        match self.sol_mode {
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
                let sol_info_cmd = SolInfoCmd::new()
                    .with_sol(self.sol_mode)
                    .with_fleet_default(self.fleet_mode)
                    .with_fit_default(self.fit_mode)
                    .with_item_default(self.item_mode);
                let mut sol_infos = Vec::with_capacity(guarded_inner_sols.len());
                for (_, guarded_inner_sol) in guarded_inner_sols {
                    // Solar systems are not touched, so that requesting refine info (which is not
                    // directly related to involved sols) does not keep them alive
                    if let Some(mut sol) = refine.internal_lock_sol(guarded_inner_sol, false).await {
                        sol_infos.push(sol.get_info(sol_info_cmd.clone()).await);
                    }
                }
                sol_infos
            }
        }
    }
}
