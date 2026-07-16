use crate::{
    cmd::AddSolCmd,
    info::{FitInfoMode, FleetInfoMode, ItemInfoMode, SolInfo, SolInfoExt, SolInfoMode},
    refine::Refine,
    sol::{SolarSystem, SolarSystemId, SolarSystemInnerGuarded},
    src::{GetSrcError, SrcAlias},
};

impl Refine {
    #[tracing::instrument(name = "sol-add", level = "trace", skip_all)]
    pub async fn add_sol(&self, src_alias: Option<&SrcAlias>, cmd: AddSolCmd) -> Result<SolarSystem<'_>, AddSolError> {
        let core_src = self.internal_get_src(src_alias).await?.get_core().clone();
        let inner_sol = self
            .tpool
            .exec_standard(move || {
                let core_sol = cmd.execute(&core_src);
                SolarSystemInnerGuarded::new(core_sol)
            })
            .await;
        let id = self.store_inner_sol(inner_sol.clone()).await;
        let sol = SolarSystem::new(self, id, inner_sol).await;
        Ok(sol)
    }
    #[tracing::instrument(name = "sol-add", level = "trace", skip_all)]
    pub async fn add_sol_and_get_info(
        &self,
        src_alias: Option<&SrcAlias>,
        cmd: AddSolCmd,
        sol_mode: SolInfoMode,
        fleet_mode: FleetInfoMode,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> Result<(SolarSystem<'_>, SolInfo), AddSolError> {
        let core_src = self.internal_get_src(src_alias).await?.get_core().clone();
        let (inner_sol, info_ext) = self
            .tpool
            .exec_standard(move || {
                let mut core_sol = cmd.execute(&core_src);
                let info_ext = SolInfoExt::try_from_core(&mut core_sol, sol_mode, fleet_mode, fit_mode, item_mode);
                let inner_sol = SolarSystemInnerGuarded::new(core_sol);
                (inner_sol, info_ext)
            })
            .await;
        let id = self.store_inner_sol(inner_sol.clone()).await;
        let info = SolInfo::from_id_and_ext(id, info_ext);
        let sol = SolarSystem::new(self, id, inner_sol).await;
        Ok((sol, info))
    }
    async fn store_inner_sol(&self, inner_sol: SolarSystemInnerGuarded) -> SolarSystemId {
        let mut id = SolarSystemId::new();
        let mut id_sol_map = self.id_sol_map.write().await;
        while id_sol_map.contains_key(&id) {
            id = SolarSystemId::new();
        }
        id_sol_map.insert(id, inner_sol);
        id
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddSolError {
    #[error("failed to get source: {0}")]
    GetSrcFailed(#[from] GetSrcError),
}
