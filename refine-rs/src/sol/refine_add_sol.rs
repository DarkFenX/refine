use crate::{
    cmd::AddSolCmd,
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
