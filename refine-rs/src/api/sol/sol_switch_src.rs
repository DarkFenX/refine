use crate::{
    FitInfoMode, FleetInfoMode, ItemInfoMode, SolInfo, SolInfoMode, SolarSystem,
    src::{SrcAlias, err::GetSrcError},
};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-swt-src", level = "trace", skip_all)]
    pub async fn switch_src(&mut self, src_alias: Option<SrcAlias>) -> Result<(), SolSwitchSrcError> {
        let src = self.refine.internal_get_src(src_alias).await?.get_core().clone();
        self.exec_standard_safe(move |core_sol| core_sol.set_src(&src)).await;
        Ok(())
    }
    #[tracing::instrument(name = "sol-swt-src", level = "trace", skip_all)]
    pub async fn switch_src_and_get_info(
        &mut self,
        src_alias: Option<SrcAlias>,
        sol_mode: SolInfoMode,
        fleet_mode: FleetInfoMode,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> Result<SolInfo, SolSwitchSrcError> {
        let sol_id = self.id;
        let src = self.refine.internal_get_src(src_alias).await?.get_core().clone();
        let sol_info = self
            .exec_standard_safe(move |core_sol| {
                core_sol.set_src(&src);
                SolInfo::from_id_and_core(sol_id, core_sol, sol_mode, fleet_mode, fit_mode, item_mode)
            })
            .await;
        Ok(sol_info)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolSwitchSrcError {
    #[error("unable to get source: {0}")]
    SrcGetFailed(#[from] GetSrcError),
}
