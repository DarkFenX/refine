use crate::{
    FitInfoMode, FleetInfoMode, ItemInfoMode, SolInfo, SolInfoMode, SolarSystem,
    src::{SrcAlias, err::GetSrcError},
};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-swt-src", level = "trace", skip_all)]
    pub async fn switch_src(&mut self, src_alias: Option<SrcAlias>) -> Result<(), SolSwitchSrcError> {
        // Variables for move
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
        let inner_src = self.refine.internal_get_src(src_alias).await?;
        let src = inner_src.get_core().clone();
        // Variables for move
        let sol_id = self.get_id();
        let src_alias = inner_src.get_alias().clone();
        let sol_info = self
            .exec_standard_safe(move |core_sol| {
                core_sol.set_src(&src);
                SolInfo::from_ids_and_core(sol_id, src_alias, core_sol, sol_mode, fleet_mode, fit_mode, item_mode)
            })
            .await;
        self.inner.set_src_alias(inner_src.get_alias().clone());
        Ok(sol_info)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolSwitchSrcError {
    #[error("unable to get source: {0}")]
    SrcGetFailed(#[from] GetSrcError),
}
